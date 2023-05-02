// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use kivio_common::{io_vec, vmem, Handle, HandleMut, Segment, SegmentMut};

#[test]
fn test_handle_segment_iovec() {

    // This tests a cycle around the conversion graph for handles and segments

    let iov = io_vec::IoVec::from_chunk_size(4, 1);

    // HandleMut
    let hm = vmem::HandleMut::from_vmem(vmem::Vmem::new_vec_u8(4));

    // -> SegmentMut
    let sm = vmem::SegmentMut::from_handle_mut(hm);

    // -> Vec<SegmentMut>
    let mut vsm = sm.try_split(&iov).unwrap();

    // Write to the elements (deref_mut<target> = mut [u8] for SegmentMut)
    vsm[0][0] = b'a';
    vsm[1][0] = b'b';
    vsm[2][0] = b'c';
    vsm[3][0] = b'd';

    // -> SegmentMut
    let sm = vmem::SegmentMut::try_from_vec_segment_mut(vsm).unwrap();

    // -> HandleMut
    let hm = vmem::HandleMut::try_from_segment_mut(sm).unwrap();

    // -> Handle
    let h = vmem::Handle::from_handle_mut(hm);

    // -> Segment
    let s = vmem::Segment::from_handle(h);

    // -> Vec<Segment>
    let mut vs = s.try_split(&iov).unwrap();

    // -> Read from it (deref<target = [u8]> for Segment)
    assert_eq!(vs[0][0], b'a');
    assert_eq!(vs[1][0], b'b');
    assert_eq!(vs[2][0], b'c');
    assert_eq!(vs[3][0], b'd');

    // Get a single segment we can convert into a Handle
    let s = vs.pop().unwrap();

    // -> Handle
    let h = vmem::Handle::from_segment(s);

    // -> Segment (merged, overlapping with what's left in vs)
    let ms = vmem::Segment::from_handle(h.clone());

    // -> Read from merged segment (deref<target = [u8]> for Segment)
    assert_eq!(ms[0], b'a');
    assert_eq!(ms[1], b'b');
    assert_eq!(ms[2], b'c');
    assert_eq!(ms[3], b'd');

    // -> HandleMut (must not work due to outstanding references in vs and ms)
    let r = vmem::HandleMut::try_from_handle(h);
    assert!(r.is_err());

    // Get rid of outstanding references, then try again -> HandleMut
    drop(vs);
    drop(ms);
    let h = r.err().unwrap().1;
    let r = vmem::HandleMut::try_from_handle(h);
    assert!(r.is_ok());
    let hm = r.ok().unwrap();

    // -> SegmentMut
    let sm = vmem::SegmentMut::from_handle_mut(hm);

    // -> Read from it (before splitting)
    assert_eq!(sm[0], b'a');
    assert_eq!(sm[1], b'b');
    assert_eq!(sm[2], b'c');
    assert_eq!(sm[3], b'd');

    // -> Vec<SegmentMut>
    let vsm = sm.try_split(&iov).unwrap();

    // Verify contents after splitting (deref<target = [u8]> for SegmentMut)
    assert_eq!(vsm[0][0], b'a');
    assert_eq!(vsm[1][0], b'b');
    assert_eq!(vsm[2][0], b'c');
    assert_eq!(vsm[3][0], b'd');

}
