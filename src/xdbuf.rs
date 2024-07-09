use std::ops::Range;

use anyhow::anyhow;

use crate::walker::Walker;

/// Structure representing an n-dimensional buffer
///
/// Reusing a single instance reduces memory allocation.
///
/// n次元のバッファを表す構造体です。
///
/// 単一のインスタンスを再利用することで、メモリの割り当てを削減できます。
#[derive(Debug, Clone)]
pub struct XDBuf<T, const D: usize> {
    buf: Vec<T>,
    size: [usize; D],
    stride: [usize; D],
}

impl<T, const D: usize> XDBuf<T, D> {
    /// Convert an index in array notation to a scalar index.
    ///
    /// 配列表記のインデックスをスカラーのインデックスに変換します。
    ///
    /// # Errors
    ///
    /// * Error if `index` is out of range.
    ///
    /// * `index`が範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    ///
    /// let index = [1, 2, 3];
    /// let scalar = buf.to_scalar_index(&index).unwrap();
    ///
    /// assert_eq!(scalar, 3*4*3 + 3*2 + 1);
    /// ```
    pub fn to_scalar_index(&self, index: &[usize; D]) -> Result<usize, anyhow::Error> {
        index.iter().zip(self.stride).try_fold(0_usize, |acc, (&i, v)| {
            acc.checked_add(i.checked_mul(v).ok_or(
                anyhow!("index is out of range")
            )?).ok_or(
                anyhow!("index is out of range")
            )
        })
    }

    /// Convert scalar index to array notation.
    ///
    /// スカラーのインデックスを配列表記に変換します。
    fn to_mul_dim_index(&self, mut scalar: usize) -> [usize; D] {
        let mut index = [0; D];

        for i in (0..D).rev() {
            index[i] = scalar / self.stride[i];
            scalar %= self.stride[i];
        }

        index
    }

    /// Checks for index integrity.
    ///
    /// インデックスの整合性をチェックします。
    pub(crate) fn validate_index(&self, index: &[usize; D]) -> Result<(), anyhow::Error> {
        let in_range = index.iter().zip(self.size.iter()).all(|(&i, &s)| i < s);

        if in_range {
            Ok(())
        } else {
            Err(anyhow!("index is out of range"))
        }
    }

    /// Computes the number of elements in a multidimensional array
    ///
    /// 多次元配列の要素数を計算します
    ///
    /// # Errors
    ///
    /// * Error if the total product of `size` exceeds the range of `usize`.///
    ///
    /// * `size`の総積が`usize`の範囲を超える場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let size = [3, 4, 5];
    /// let total_size = XDBuf::<i32, 3>::calc_total_size(&size).unwrap();
    ///
    /// assert_eq!(total_size, 60);
    /// ```
    pub fn calc_total_size(size: &[usize; D]) -> Result<usize, anyhow::Error> {
        if size.iter().any(|&v| v == 0) {
            return Err(anyhow!("size is out of range"));
        }

        size.iter().try_fold(1_usize, |acc, &v| {
            acc.checked_mul(v).ok_or(
                anyhow!("size is out of range")
            )
        })
    }

    /// Calculates the number of elements each dimension of a multidimensional array has.
    ///
    /// 多次元配列の各次元が持つ要素数を計算します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let size = [3, 4, 5];
    /// let stride = XDBuf::<i32, 3>::calc_dim_stride(&size).unwrap();
    ///
    /// assert_eq!(stride, [1, 3, 12]);
    /// ```
    pub fn calc_dim_stride(size: &[usize; D]) -> Result<[usize; D], anyhow::Error> {
        let mut stride = [1_usize; D];
        for i in 0..D {
            for j in 0..i {
                stride[i] = stride[i].checked_mul(size[j]).ok_or(
                    anyhow!("size is out of range")
                )?;
            }
        }

        Ok(stride)
    }

    /// Generate a new `XDBuf`.
    ///
    /// Allocates the specified amount of elements for each dimension and fills them with initial values.
    ///
    /// 新しい`XDBuf`を生成します。
    ///
    /// それぞれの次元について指定した分の要素を確保し、初期値で埋めます。
    ///
    /// # Errors
    ///
    /// * Error if the total product of `size` exceeds the range of `usize`.
    ///
    /// * `size`の総積が`usize`の範囲を超える場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let size = [3, 4, 5];
    /// let buf = XDBuf::<i32, 3>::new(size, 0).unwrap();
    /// ```
    pub fn new(size: [usize; D], initial_value: T) -> Result<Self, anyhow::Error>
    where
        T: Clone,
    {
        let total_size = Self::calc_total_size(&size)?;

        let mut buf = Vec::with_capacity(total_size);
        buf.resize(total_size, initial_value);

        Ok(Self {
            buf,
            size,
            stride: Self::calc_dim_stride(&size)?,
        })
    }

    /// Generate an `XDBuf` from a `Vec<T>`.
    ///
    /// Initialize it using the given `Vec<T>` and treat it as a `Vec` of dimension `D`.
    ///
    /// `Vec<T>`から`XDBuf`を生成します。
    ///
    /// 与えられた`Vec<T>`を使用して初期化し、`D`次元の`Vec`として扱います。
    ///
    /// # Errors
    ///
    /// * Error if the length of `initial_vec` does not match the total product of `size`.
    /// * Error if the total product of `size` exceeds the range of `usize`.
    ///
    /// * `initial_vec`の長さが`size`の総積と一致しない場合エラーになります。
    /// * `size`の総積が`usize`の範囲を超える場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let size = [3, 4, 5];
    /// let initial_vec = vec![0; 60];
    /// let buf = XDBuf::<i32, 3>::new_with_vec(size, initial_vec).unwrap();
    /// ```
    ///
    /// ```should_panic
    /// use xdbuf::XDBuf;
    ///
    /// let size = [3, 4, 5];
    /// let initial_vec = vec![0; 59]; // 59 != 3 * 4 * 5
    /// let buf = XDBuf::<i32, 3>::new_with_vec(size, initial_vec).unwrap(); // panic!
    /// ```
    pub fn new_with_vec(size: [usize; D], initial_vec: Vec<T>) -> Result<Self, anyhow::Error> {
        let total_size = Self::calc_total_size(&size)?;

        if initial_vec.len() != total_size {
            return Err(anyhow!("initial_vec length is not equal to total_size"));
        }

        Ok(Self {
            buf: initial_vec,
            size,
            stride: Self::calc_dim_stride(&size)?,
        })
    }

    /// Initialize buffers.
    ///
    /// The internally allocated capacity is not affected in the shrink direction.
    ///
    /// バッファを初期化します。
    ///
    /// 内部に割り当てられた容量には縮小方向への影響を与えません。
    ///
    /// # Errors
    ///
    /// * Error if the total product of `size` exceeds the range of `usize`.
    ///
    /// * `size`の総積が`usize`の範囲を超える場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let mut buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    /// assert_eq!(buf.len(), 60);
    /// assert_eq!(buf.get(0), Some(&0));
    ///
    /// buf.init([1, 2, 3], 1).unwrap();
    /// assert_eq!(buf.len(), 6);
    /// assert_eq!(buf.get(0), Some(&1));
    /// ```
    pub fn init(&mut self, size: [usize; D], initial_value: T) -> Result<(), anyhow::Error>
    where
        T: Clone,
    {
        self.size = size;
        self.stride = Self::calc_dim_stride(&size)?;

        self.buf.clear();
        self.buf.resize(Self::calc_total_size(&size)?, initial_value);

        Ok(())
    }

    /// Initialize the buffer from `Vec<T>`.
    ///
    /// The internally allocated capacity is not affected in the shrink direction.
    ///
    /// `Vec<T>`からバッファを初期化します。
    ///
    /// 内部に割り当てられた容量には縮小方向への影響を与えません。
    ///
    /// # Errors
    ///
    /// * Error if the total product of `size` exceeds the range of `usize`.
    ///
    /// * `size`の総積が`usize`の範囲を超える場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let mut buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    /// assert_eq!(buf.len(), 60);
    ///
    /// let initial_vec = vec![1; 6];
    /// buf.init_with_vec([1, 2, 3], initial_vec).unwrap();
    /// assert_eq!(buf.len(), 6);
    /// ```
    ///
    /// ```should_panic
    /// use xdbuf::XDBuf;
    ///
    /// let mut buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    /// assert_eq!(buf.len(), 60);
    ///
    /// let initial_vec = vec![1; 5]; // 5 != 1 * 2 * 3
    /// buf.init_with_vec([1, 2, 3], initial_vec).unwrap(); // panic!
    /// ```
    pub fn init_with_vec(&mut self, size: [usize; D], mut initial_vec: Vec<T>) -> Result<(), anyhow::Error> {
        self.size = size;
        self.stride = Self::calc_dim_stride(&size)?;

        if initial_vec.len() != Self::calc_total_size(&size)? {
            return Err(anyhow!("initial_vec length is not equal to total_size"));
        }

        self.buf.clear();
        self.buf.append(&mut initial_vec);

        Ok(())
    }

    /// Get a reference to the element specified by `index`.
    ///
    /// Returns `None` if `index` is out of range.
    ///
    /// `index`で指定された要素の参照を取得します。
    ///
    /// `index`が範囲外の場合は`None`を返します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let initial_vec = (1..=60).collect::<Vec<i32>>();
    /// let buf = XDBuf::<i32, 3>::new_with_vec([3, 4, 5], initial_vec).unwrap();
    ///
    /// assert_eq!(buf.get(0), Some(&1));
    /// assert_eq!(buf.get(1), Some(&2));
    /// assert_eq!(buf.get(59), Some(&60));
    /// assert_eq!(buf.get(60), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.buf.len() {
            return None;
        }

        self.buf.get(index)
    }

    /// Get a variable reference to the element specified by `index`.
    ///
    /// Returns `None` if `index` is out of range.
    ///
    /// `index`で指定された要素の可変参照を取得します。
    ///
    /// `index`が範囲外の場合は`None`を返します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let mut initial_vec = (1..=60).collect::<Vec<i32>>();
    /// let mut buf = XDBuf::<i32, 3>::new_with_vec([3, 4, 5], initial_vec).unwrap();
    ///
    /// assert_eq!(buf.get(0), Some(&1));
    ///
    /// let value = buf.get_mut(0).unwrap();
    /// *value = 100;
    ///
    /// assert_eq!(buf.get(0), Some(&100));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.buf.len() {
            return None;
        }

        Some(&mut self.buf[index])
    }

    /// Set `value` to the element specified by `index`.
    ///
    /// `index`で指定された要素に`value`を設定します。
    ///
    /// # Errors
    ///
    /// * Error if `index` is out of range.
    ///
    /// * `index`が範囲外の場合エラーになります。
    pub fn set(&mut self, index: usize, value: T) -> Result<(), anyhow::Error> {
        if index >= self.buf.len() {
            return Err(anyhow!("index is out of range"));
        }

        self.buf[index] = value;

        Ok(())
    }

    /// Generates a `Walker` with the specified `index` as its initial position.
    ///
    /// 指定された`index`を初期位置として`Walker`を生成します。
    ///
    /// # Errors
    ///
    /// * Error if `index` is out of range.
    ///
    /// * `index`が範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let mut buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    /// let walker = buf.walker_from([0, 0, 0]).unwrap();
    /// ```
    pub fn walker_from(&self, index: [usize; D]) -> Result<Walker<T, D>, anyhow::Error> {
        self.validate_index(&index)?;

        let scalar = self.to_scalar_index(&index)?;

        Ok(
            Walker {
                buf_into: self,
                current_index: scalar,
            }
        )
    }

    /// Returns the number of elements in the buffer.
    ///
    /// バッファの要素数を返します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    /// assert_eq!(buf.len(), 60);
    /// ```
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns a range of buffer indices.
    ///
    /// バッファのインデックスの範囲を返します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    /// assert_eq!(buf.idx_range(), 0..60);
    /// ```
    pub fn idx_range(&self) -> Range<usize> {
        0..self.buf.len()
    }

    /// Returns the number of elements each buffer dimension has.
    ///
    /// バッファの各次元が持つ要素数を返します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();    ///
    /// assert_eq!(buf.stride(), &[1, 3, 12]);
    /// ```
    pub fn stride(&self) -> &[usize; D] {
        &self.stride
    }

    /// Reduce buffer capacity as much as possible.
    ///
    /// バッファが確保しているメモリ容量をできるだけ縮小します。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::XDBuf;
    ///
    /// let mut buf = XDBuf::<i32, 3>::new([3, 4, 5], 0).unwrap();
    ///
    /// // 確保されている容量よりも小さい要素数をに初期化
    /// buf.init([1, 2, 3], 1).unwrap();
    ///
    /// buf.shrink_to_fit();
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.buf.shrink_to_fit();
    }
}
