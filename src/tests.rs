#[cfg(test)]
mod end_to_end {
    use crate::{exec::ExecState, parser, scanner};

    fn assert_print(text: &str, expected_output: &str) {
        let mut state = ExecState::new_test();
        let tokens = scanner::scan(text, 0).unwrap();
        let program = parser::parse(tokens).unwrap();
        state.exec(&program).unwrap();
        state.assert_output(expected_output);
    }

    #[test]
    fn hello() {
        assert_print("var a = 3; print(a);", "3\n");
    }

    #[test]
    fn nested_scopes() {
        let text = r#"
var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a; // inner a
    print b; // outer b
    print c; // global c
    print "";
  }
  print a; // outer a
  print b; // outer b
  print c; // global c
  print "";
}
print a;
print b;
print c;"#;

        let output = r#"inner a
outer b
global c

outer a
outer b
global c

global a
global b
global c
"#;

        assert_print(text, output);
    }

    #[test]
    fn test_if() {
        let text = r#"
var a = 1;
if (a)
    print "ok";
else
    print "fail";
"#;

        let output = r#"ok
"#;

        assert_print(text, output);
    }

    #[test]
    fn test_while() {
        let text = r#"
var a = 1;
while (a < 5) {
    print a;
    a = a + 1;
}
"#;

        let output = r#"1
2
3
4
"#;

        assert_print(text, output);
    }
}
