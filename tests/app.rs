use xloc;

const DATA_PATH: &'static str = "tests/data";

#[test]
fn default_app() {
    let app = xloc::App::default();
    assert_eq!(app.get_njobs(), 1);
    assert_eq!(app.get_words(), false);
}

#[test]
fn new_app() {
    let app = xloc::App::new(3, true);
    assert_eq!(app.get_njobs(), 3);
    assert_eq!(app.get_words(), true);
}

#[test]
fn count_lines_dir() {
    let app = xloc::App::default();
    let base_path = String::from(DATA_PATH);

    let result = app.count(&base_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 45);
}

#[test]
fn count_words_dir() {
    let app = xloc::App::new(1, true);
    let base_path = String::from(DATA_PATH);

    let result = app.count(&base_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 120);
}

#[test]
fn count_lines_file() {
    let app = xloc::App::default();
    let base_path = String::from(DATA_PATH);

    // A vec containing tuples of file name and actual line count
    let data_vec = vec![("/data.py", 17), ("/data.rs", 17), ("/data.txt", 11)];

    // Iterate over data vec and make sure each files line counts match
    for data in data_vec {
        let mut path = base_path.clone();
        path.push_str(data.0);

        let result = app.count(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data.1);
    }
}

#[test]
fn count_words_file() {
    let app = xloc::App::new(1, true);
    let base_path = String::from(DATA_PATH);

    // A vec containing tuples of file name and actual word count
    let data_vec = vec![("/data.py", 40), ("/data.rs", 36), ("/data.txt", 44)];

    // Iterate over data vec and make sure each files word counts match
    for data in data_vec {
        let mut path = base_path.clone();
        path.push_str(data.0);

        let result = app.count(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data.1);
    }
}

#[test]
fn count_lines_dir_bad_path() {
    let app = xloc::App::default();
    let base_path = String::from("fake_dir");

    let result = app.count(&base_path);
    assert!(result.is_err());

    let e = result.unwrap_err().to_string();
    let error_msgs = vec![
        "cannot find the path specified",
        "No such file or directory",
    ];

    assert!(e.contains(error_msgs[0]) | e.contains(error_msgs[1]));
}

#[test]
fn set_njobs() {
    let mut app = xloc::App::default();
    assert_eq!(app.get_njobs(), 1);
    assert_eq!(app.set_njobs(3), 3);
    assert_eq!(app.get_njobs(), 3);
}

#[test]
fn set_words() {
    let mut app = xloc::App::default();
    assert_eq!(app.get_words(), false);
    assert_eq!(app.set_words(true), true);
    assert_eq!(app.get_words(), true);
}

#[test]
fn derives() {
    // Create 2 different apps
    let app = xloc::App::default();
    let app2 = xloc::App::new(1, true);

    // Test Clone
    let cloned = app.clone();

    // Test Eq and Partialeq
    assert_eq!(app, cloned);
    assert!(app.eq(&cloned));
    assert!(app.ne(&app2));
    assert!(app2.ne(&cloned));
}
