use pyo3::prelude::*;

type TS1 = (String,);
type TS2 = (String,String);
type TS3 = (String,String,String);
type TS4 = (String,String,String,String);
type TS5 = (String,String,String,String,String);
type TS6 = (String,String,String,String,String,String);
type TS7 = (String,String,String,String,String,String,String);
type TS8 = (String,String,String,String,String,String,String,String);



#[pyfunction]
fn cartesian_1d(l1: Vec<String>) -> PyResult<Vec<TS1>> {
    let collector = searchsorted::cartesian_1d(l1);
    Ok(collector)
}
#[pyfunction]
fn cartesian_2d(l1: Vec<String>, l2: Vec<String>) -> PyResult<Vec<TS2>> {
    let collector = searchsorted::cartesian_2d(l1, l2);
    Ok(collector)
}
#[pyfunction]
fn cartesian_3d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>) -> PyResult<Vec<TS3>> {
    let collector = searchsorted::cartesian_3d(l1, l2, l3);
    Ok(collector)
}
#[pyfunction]
fn cartesian_4d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>) -> PyResult<Vec<TS4>> {
    let collector = searchsorted::cartesian_4d(l1, l2, l3, l4);
    Ok(collector)
}
#[pyfunction]
fn cartesian_5d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>) -> PyResult<Vec<TS5>> {
    let collector = searchsorted::cartesian_5d(l1,l2,l3,l4,l5);
    Ok(collector)
}
#[pyfunction]
fn cartesian_6d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>, l6: Vec<String>) -> PyResult<Vec<TS6>> {
    let collector = searchsorted::cartesian_6d(l1,l2,l3,l4,l5,l6);
    Ok(collector)
}

#[pyfunction]
fn cartesian_7d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>, l6: Vec<String>, l7: Vec<String>) -> PyResult<Vec<TS7>> {
    let collector = searchsorted::cartesian_7d(l1,l2,l3,l4,l5,l6,l7);
    Ok(collector)
}

#[pyfunction]
fn cartesian_8d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>, l6: Vec<String>, l7: Vec<String>, l8: Vec<String>) -> PyResult<Vec<TS8>> {
    let collector = searchsorted::cartesian_8d(l1,l2,l3,l4,l5,l6,l7,l8);
    Ok(collector)
}

#[pyfunction]
fn searchsorted_1d(dense_list: Vec<TS1>, index_list: Vec<TS1>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_1d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_2d(dense_list: Vec<TS2>, index_list: Vec<TS2>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_2d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_3d(dense_list: Vec<TS3>, index_list: Vec<TS3>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_3d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_4d(dense_list: Vec<TS4>, index_list: Vec<TS4>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_4d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_5d(dense_list: Vec<TS5>, index_list: Vec<TS5>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_5d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_6d(dense_list: Vec<TS6>, index_list: Vec<TS6>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_6d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_7d(dense_list: Vec<TS7>, index_list: Vec<TS7>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_7d(dense_list, index_list);
    Ok(location)
}

#[pyfunction]
fn searchsorted_8d(dense_list: Vec<TS8>, index_list: Vec<TS8>) -> PyResult<Vec<i64>> {
    let location = searchsorted::searchsorted_8d(dense_list, index_list);
    Ok(location)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cartesian_1d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_2d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_3d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_4d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_5d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_6d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_7d, m)?)?;
    m.add_function(wrap_pyfunction!(cartesian_8d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_1d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_2d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_3d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_4d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_5d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_6d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_7d, m)?)?;
    m.add_function(wrap_pyfunction!(searchsorted_8d, m)?)?;
    Ok(())
}
