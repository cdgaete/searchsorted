use itertools::iproduct;
use std::collections::HashMap;

type TS1 = (String,);
type TS2 = (String,String);
type TS3 = (String,String,String);
type TS4 = (String,String,String,String);
type TS5 = (String,String,String,String,String);
type TS6 = (String,String,String,String,String,String);
type TS7 = (String,String,String,String,String,String,String);
type TS8 = (String,String,String,String,String,String,String,String);




pub fn cartesian_1d(l1: Vec<String>) -> Vec<TS1> {
    let mut collector = Vec::new();
    for s1 in iproduct!(l1) {
        collector.push((s1,));
    };
    collector
}

pub fn cartesian_2d(l1: Vec<String>, l2: Vec<String>) -> Vec<TS2> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2) {
        collector.push(tuple);
    };
    collector
}

pub fn cartesian_3d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>) -> Vec<TS3> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3) {
        collector.push(tuple);
    };
    collector
}

pub fn cartesian_4d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>) -> Vec<TS4> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3,l4) {
        collector.push(tuple);
    };
    collector
}

pub fn cartesian_5d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>) -> Vec<TS5> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3,l4,l5) {
        collector.push(tuple);
    };
    collector
}

pub fn cartesian_6d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>, l6: Vec<String>) -> Vec<TS6> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3,l4,l5,l6) {
        collector.push(tuple);
    };
    collector
}

pub fn cartesian_7d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>, l6: Vec<String>, l7: Vec<String>) -> Vec<TS7> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3,l4,l5,l6,l7) {
        collector.push(tuple);
    };
    collector
}

pub fn cartesian_8d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>, l4: Vec<String>, l5: Vec<String>, l6: Vec<String>, l7: Vec<String>, l8: Vec<String>) -> Vec<TS8> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3,l4,l5,l6,l7,l8) {
        collector.push(tuple);
    };
    collector
}

pub fn searchsorted_1d(dense_list: Vec<TS1>, index_list: Vec<TS1>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_2d(dense_list: Vec<TS2>, index_list: Vec<TS2>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_3d(dense_list: Vec<TS3>, index_list: Vec<TS3>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_4d(dense_list: Vec<TS4>, index_list: Vec<TS4>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_5d(dense_list: Vec<TS5>, index_list: Vec<TS5>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_6d(dense_list: Vec<TS6>, index_list: Vec<TS6>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_7d(dense_list: Vec<TS7>, index_list: Vec<TS7>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}

pub fn searchsorted_8d(dense_list: Vec<TS8>, index_list: Vec<TS8>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}
