use anyhow::anyhow;

use crate::XDBuf;

/// `XDBuf`におけるインデックス操作を行うための構造体
pub struct Walker<'a, T, const D: usize> {
    pub(super) buf_into: &'a XDBuf<T, D>,
    pub(super) current_index: usize,
}

impl<'a, T, const D: usize> Walker<'a, T, D> {
    /// 現在のインデックスを返します。
    pub fn index(&self) -> usize {
        self.current_index
    }

    /// 現在のインデックスから`step`を加算したインデックスを返します。
    ///
    /// # Errors
    ///
    /// * 移動先のインデックスが範囲外の場合エラーになります。
    /// * バッファの長さが`isize::MAX`を超えている場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let walker = buf.walker_from([1, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 4);
    ///
    /// let next_index = walker.index_(&[1, 0]).unwrap();
    /// assert_eq!(next_index, 5);
    ///
    /// let next_index = walker.index_(&[0, 1]).unwrap();
    /// assert_eq!(next_index, 7);
    ///```
    pub fn index_(&self, step: &[isize; D]) -> Result<usize, anyhow::Error> {
        let mut increment = 0_isize;

        step.iter().zip(self.buf_into.stride()).try_for_each(
            |(&step, &stride)| {
                let stride = stride.try_into()?;

                increment = increment
                    .checked_add(
                        step.checked_mul(stride)
                            .ok_or(anyhow!(""))?
                    ).ok_or(anyhow!(""))?;

                Ok(())
            }
        ).map_err(
            |_: anyhow::Error| anyhow!("Calculating indexes using step can only be used when the buffer length is less than or equal to usize::MAX.")
        )?;

        let current_index: isize = self.current_index.try_into()?;
        let next_index: usize = current_index.checked_add(increment).ok_or(
            anyhow::anyhow!("Index out of range")
        )?.try_into()?;

        if next_index >= self.buf_into.len() {
            return Err(anyhow::anyhow!("Index out of range"));
        }

        Ok(next_index)
    }

    /// 現在のインデックスから`step`を加算したインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 移動先のインデックスが範囲外の場合エラーになります。
    /// * バッファの長さが`isize::MAX`を超えている場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([1, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// walker.as_(&[1, 1]).unwrap();
    /// assert_eq!(walker.index(), 8);
    ///
    /// walker.as_(&[-1, 0]).unwrap();
    /// assert_eq!(walker.index(), 7);
    ///```
    pub fn as_(&mut self, step: &[isize; D]) -> Result<&mut Self, anyhow::Error> {
        self.current_index = self.index_(step)?;
        Ok(self)
    }

    /// 現在のインデックスから`step`を加算したインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 移動先のインデックスが範囲外の場合エラーになります。
    /// * バッファの長さが`isize::MAX`を超えている場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([1, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// let walker = walker.into_(&[1, 1]).unwrap();
    /// assert_eq!(walker.index(), 8);
    ///
    /// let walker = walker.into_(&[-1, 0]).unwrap();
    /// assert_eq!(walker.index(), 7);
    ///```
    pub fn into_(mut self, step: &[isize; D]) -> Result<Self, anyhow::Error> {
        self.as_(step)?;
        Ok(self)
    }

    /// 次のインデックスを返します。
    ///
    /// # Errors
    ///
    /// * 移動先のインデックスが範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let walker = buf.walker_from([2, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 5);
    ///
    /// let next_index = walker.next_index().unwrap();
    /// assert_eq!(next_index, 6);
    ///```
    pub fn next_index(&self) -> Result<usize, anyhow::Error> {
        let next_index = self.current_index.checked_add(1).ok_or(
            anyhow::anyhow!("Index out of range")
        )?;

        if next_index >= self.buf_into.len() {
            return Err(anyhow::anyhow!("Index out of range"));
        }

        Ok(next_index)
    }


    /// 次のインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 次のインデックスが範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([2, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// walker.as_next().unwrap();
    /// assert_eq!(walker.index(), 6);
    /// ```
    pub fn as_next(&mut self) -> Result<&mut Self, anyhow::Error> {
        self.current_index = self.next_index()?;
        Ok(self)
    }

    /// 次のインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 次のインデックスが範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([2, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// let walker = walker.into_next().unwrap();
    /// assert_eq!(walker.index(), 6);
    /// ```
    pub fn into_next(mut self) -> Result<Self, anyhow::Error> {
        self.as_next()?;
        Ok(self)
    }

    /// 前のインデックスを返します。
    ///
    /// # Errors
    ///
    /// * 前のインデックスが範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let walker = buf.walker_from([2, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 5);
    ///
    /// let prev_index = walker.prev_index().unwrap();
    /// assert_eq!(prev_index, 4);
    /// ```
    pub fn prev_index(&self) -> Result<usize, anyhow::Error> {
        let prev_index = self.current_index.checked_sub(1).ok_or(
            anyhow::anyhow!("Index out of range")
        )?;

        if prev_index >= self.buf_into.len() {
            return Err(anyhow::anyhow!("Index out of range"));
        }

        Ok(prev_index)
    }

    /// 前のインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 前のインデックスが範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};    ///
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([2, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// walker.as_prev().unwrap();
    /// assert_eq!(walker.index(), 4);
    /// ```
    pub fn as_prev(&mut self) -> Result<&mut Self, anyhow::Error> {
        self.current_index = self.prev_index()?;
        Ok(self)
    }

    /// 前のインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 前のインデックスが範囲外の場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};    ///
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([2, 1]).unwrap();
    ///
    /// //[0, 1, 2
    /// // 3, 4, 5
    /// // 6, 7, 8]
    ///
    /// let walker = walker.into_prev().unwrap();
    /// assert_eq!(walker.index(), 4);
    /// ```
    pub fn into_prev(mut self) -> Result<Self, anyhow::Error> {
        self.as_prev()?;
        Ok(self)
    }

    /// 現在のインデックス以降の要素を走査し、条件を満たす最初のインデックスを返します。
    ///
    /// # Errors
    ///
    /// * 最後の要素まで条件を満たす要素が見つからない場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let walker = buf.walker_from([0, 0]).unwrap();
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 0);
    /// assert_eq!(buf.get(walker.index()), Some(&1));
    ///
    /// let index = walker.index_until(|&x, _i| x == 5).unwrap();
    /// assert_eq!(index, 4);
    /// ```
    ///
    /// ```should_panic
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let walker = buf.walker_from([0, 0]).unwrap();
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 0);
    /// assert_eq!(buf.get(walker.index()), Some(&1));
    ///
    /// let index = walker.index_until(|&x, _i| x < 0).unwrap(); // panic!
    /// ```
    pub fn index_until(&self, f: impl Fn(&T, usize) -> bool) -> Result<usize, anyhow::Error> {
        let mut index = self.current_index;
        // indexのインクリメント時に境界チェックをしているのでunwrapは安全
        while !f(self.buf_into.get(index).unwrap(), index) {
            index = index.checked_add(1).ok_or(
                anyhow!("No element satisfying the function exists")
            )?;

            if index >= self.buf_into.len() {
                return Err(anyhow!("No element satisfying the function exists"));
            }
        }

        Ok(index)
    }

    /// 現在のインデックス以降の要素を走査し、条件を満たす最初のインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 最後の要素まで条件を満たす要素が見つからない場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([0, 0]).unwrap();
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 0);
    ///
    /// walker.as_until(|&x, _i| x == 5).unwrap();
    /// assert_eq!(walker.index(), 4);
    /// ```
    pub fn as_until(&mut self, f: impl Fn(&T, usize) -> bool) -> Result<&mut Self, anyhow::Error> {
        self.current_index = self.index_until(f)?;
        Ok(self)
    }

    /// 現在のインデックス以降の要素を走査し、条件を満たす最初のインデックスに移動します。
    ///
    /// # Errors
    ///
    /// * 最後の要素まで条件を満たす要素が見つからない場合エラーになります。
    ///
    /// # Example
    ///
    /// ```
    /// use xdbuf::{XDBuf, Walker};
    ///
    /// let initial_vec = (1..=9).collect::<Vec<_>>();
    /// let buf = XDBuf::new_with_vec([3, 3], initial_vec).unwrap();
    /// let mut walker = buf.walker_from([0, 0]).unwrap();
    ///
    /// let current_index = walker.index();
    /// assert_eq!(current_index, 0);
    ///
    /// let walker = walker.into_until(|&x, _i| x == 5).unwrap();
    /// assert_eq!(walker.index(), 4);
    /// ```
    pub fn into_until(mut self, f: impl Fn(&T, usize) -> bool) -> Result<Self, anyhow::Error> {
        self.as_until(f)?;
        Ok(self)
    }
}
