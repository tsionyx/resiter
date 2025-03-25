use crate::errors::GetErrors as _;

/// Extension trait for `Iterator<Item = Result<(), E>>` (or other `T` _isomorphic_ to unit `()`)
/// to fold into a single value.
pub trait FoldUnit<T, E>: Iterator<Item = Result<T, E>> {
    /// Stop on the first [`Err`] output by skipping all the consequent items.
    ///
    /// # Examples
    ///
    /// Implemented for `Result<(), E>` the following two implementations are roughly equivalent:
    ///
    /// ```
    /// # use resiter::FoldUnit;
    ///
    /// type UnitResult<E> = Result<(), E>;
    ///
    /// fn process_side_effects<E>(mut iter: impl Iterator<Item=UnitResult<E>>) -> UnitResult<E> {
    ///   for x in iter {
    ///     x?;
    ///   }
    ///   Ok(())
    /// }
    ///
    /// fn process_side_effects_with_fail_fast<E>(mut iter: impl Iterator<Item=UnitResult<E>>) -> UnitResult<E> {
    ///   iter.fail_fast()
    /// }
    ///
    /// let values = vec![Ok(()), Ok(()), Err("error1"), Ok(()), Err("error2")];
    /// assert_eq!(process_side_effects(values.clone().into_iter()), Err("error1"));
    /// assert_eq!(process_side_effects_with_fail_fast(values.into_iter()), Err("error1"));
    /// ```
    ///
    /// Also works for `Result<T, E>` where `T` is the other unit type (may be constructed `From<()>`):
    /// ```
    /// # use resiter::FoldUnit;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Unit;
    /// impl From<()> for Unit {
    ///     fn from(_: ()) -> Self {
    ///         Unit
    ///     }
    /// }
    ///
    /// let values = vec![Ok(Unit), Ok(Unit), Err("error1"), Ok(Unit), Err("error2")];
    /// assert_eq!(values.into_iter().fail_fast(), Err("error1"));
    /// ```
    ///
    /// See also for somewhat similar functionality: [`Errors`][crate::errors::Errors];
    fn fail_fast(&mut self) -> Result<T, E>;

    /// Process the iterator till the end, ignoring all but the last [`Err`].
    ///
    /// This is mostly useful for the stateful calculations
    /// to ensure independent processing of *every** individual [`Self::Item`]
    /// is done even if some of them are erroneous.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use resiter::FoldUnit;
    /// let mut acc = vec![];
    /// let mut do_side_effect = |i: Result<usize, _>| {
    ///     i.map(|i| {
    ///         println!("{} is a usize", i);
    ///         acc.push(i);
    ///     })
    /// };
    ///
    /// let values = vec!["1", "2", "foo", "4", "5"];
    /// let res = values
    ///     .iter().copied()
    ///     .map(|e| do_side_effect(usize::from_str(e).map_err(|_| e)))
    ///     .last_err();
    /// assert_eq!(acc.as_slice(), &[1, 2, 4, 5]);
    /// assert_eq!(res, Err("foo"));
    /// ```
    ///
    /// Also works for `Result<T, E>` where `T` is the other unit type (may be constructed `From<()>`):
    /// ```
    /// # use resiter::FoldUnit;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Unit;
    /// impl From<()> for Unit {
    ///     fn from(_: ()) -> Self {
    ///         Unit
    ///     }
    /// }
    ///
    /// let values = vec![Ok(Unit), Ok(Unit), Err("error1"), Ok(Unit), Err("error2")];
    /// assert_eq!(values.into_iter().last_err(), Err("error2"));
    /// ```
    ///
    /// See also for somewhat similar functionality:
    /// - [`WhileOk`][crate::while_ok::WhileOk];
    /// - [`OnOkDo`][crate::onok::OnOkDo];
    fn last_err(&mut self) -> Result<T, E>;
}

impl<I, T, E> FoldUnit<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
    T: From<()>,
{
    fn fail_fast(&mut self) -> Result<T, E> {
        self.errors().next().map_or_else(|| Ok(().into()), Err)
    }

    fn last_err(&mut self) -> Result<T, E> {
        self.errors().last().map_or_else(|| Ok(().into()), Err)
    }
}
