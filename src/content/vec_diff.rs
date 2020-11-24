use serde::{Serialize, Deserialize};

// TODO: look into copy-and-insert based diff systems?
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Op<T> {
    Insert(Vec<T>), // insert Vec<T> at index
    Delete(usize),  // delete the next n items
    Equal(usize),   // skip the next n items.
}

/// Calculates the diff of a vec of items.
/// Uses the Myers diffing algorithm
/// Applies pre-processing steps to increase efficiency,
/// applies post-processing steps to increase readability.
/// Note that the diff must cover the whole sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecDiff<T>(Vec<Op<T>>) where T: PartialEq + Clone;

// make t a slice type?

impl<T> VecDiff<T> where T: PartialEq + Clone {
    // TODO: pre and post processing steps.
    // see: https://github.com/google/diff-match-patch
    // https://neil.fraser.name/writing/diff/myers.pdf
    // and https://neil.fraser.name/writing/diff/ (especially)

    fn new(prefix: usize, postfix: usize, middle: Vec<Op<T>>) -> VecDiff<T> {
        let mut diff = vec![Op::Equal(prefix)];
        diff.append(&mut middle.to_vec());
        diff.push(Op::Equal(postfix));
        return VecDiff(diff);
    }

    fn common_prefix(prev: &[T], next: &[T], ) -> usize {
        let (mut prev_iter, mut next_iter) = (prev.iter(), next.iter());
        let mut len = 0;

        while let (Some(p), Some(n)) = (prev_iter.next(), next_iter.next()) {
            if p == n { len += 1; } else { break; }
        }
        return len;
    }

    fn common_postfix(prev: &[T], next: &[T], ) -> usize {
        let (mut prev_iter, mut next_iter) = (prev.iter().rev(), next.iter().rev());
        let mut len = 0;

        while let (Some(p), Some(n)) = (prev_iter.next(), next_iter.next()) {
            if p == n { len += 1; } else { break; }
        }
        return len;
    }

    fn double_edit(prev: &[T], next: &[T]) -> Option<Vec<Op<T>>> {
        let (long, short) = if prev.len() > next.len() {
            (prev, next)
        } else {
            (next, prev)
        };

        let sub_slice = long.windows(short.len()).position(|w| w == short)?;

        let front  = &long[..sub_slice];
        let middle = Op::<T>::Equal(short.len());
        let back   = &long[sub_slice + short.len()..];

        return if prev.len() > next.len() {
            Some(vec![Op::<T>::Delete(front.len()), middle, Op::<T>::Delete(back.len())])
        } else {
            Some(vec![Op::Insert(front.to_vec()), middle, Op::Insert(back.to_vec())])
        }
    }

    // TODO: move pre-processing to different function?

    pub fn make(prev: &[T], next: &[T]) -> VecDiff<T> {
        // if they're equal, there's no change...
        if prev == next { return VecDiff(vec![]) }

        // trim any matching data at the start and end of the slices.
        let prefix = VecDiff::common_prefix(prev, next);
        let prev = &prev[prefix..];
        let next = &next[prefix..];

        let postfix = VecDiff::common_postfix(prev, next);
        let prev = &prev[..prev.len() - postfix];
        let next = &next[..next.len() - postfix];

        // single insertions and deletions are easy to handle at this point.
        if prev.is_empty() {
            return VecDiff::new(prefix, postfix, vec![Op::Insert(next.to_owned())]);
        } else if next.is_empty() {
            return VecDiff::new(prefix, postfix, vec![Op::Delete(prev.len())]);
        }

        // double insertions are a bit more complicated...
        if let Some(edits) = VecDiff::double_edit(prev, next) {
            return VecDiff::new(prefix, postfix, edits);
        }

        // TODO: implement half-length preprocessing step.
        // TODO: implement 'An O(ND) Difference Algorithm and Its Variations'
        // TODO: post-processing cleanup

        todo!()
    }

    pub fn apply(&self, prev: &Vec<T>) -> VecDiff<T> {
        todo!()
    }
}
