import pytest

from tests.rlox import rlox
from datetime import datetime

def test_should_call_native_function():
    source = 'print clock();'

    result, stdout, _ = rlox.run(source)
    lox_time = int(stdout)
    actual_time = datetime.now().timestamp()

    assert result == rlox.SUCCESS
    assert lox_time == pytest.approx(actual_time)

def test_should_call_function():
    source = 'fun foo()                         \n' \
             '{                                 \n' \
             '    print "foo";                  \n' \
             '}                                 \n' \
             '                                  \n' \
             'foo();'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"foo"'])

def test_should_return_value_from_function():
    source = 'fun foo()                         \n' \
             '{                                 \n' \
             '    return 42;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'print foo();'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['42'])

def test_should_call_function_with_nested_variables():
    source = 'fun foo()                         \n' \
             '{                                 \n' \
             '    var a = 42;                   \n' \
             '    var b = a + 18;               \n' \
             '    return a + b;                 \n' \
             '}                                 \n' \
             '                                  \n' \
             'print foo();'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['102'])

def test_should_shadow_variables_in_functions():
    source = 'var a = "global";     \n' \
             'fun test() {          \n' \
             '    var a = "local";  \n' \
             '    print a;          \n' \
             '}                     \n' \
             'test();               \n' \
             'print a;              \n'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"local"', '"global"'])

def test_should_allow_local_function():
    source =  'fun makeCounter() {            \n' \
              '  var i = 0;                   \n' \
              '  fun count() {                \n' \
              '    i = i + 1;                 \n' \
              '    print i;                   \n' \
              '  }                            \n' \
              '  return count;                \n' \
              '}                              \n' \
              'var counter = makeCounter();   \n' \
              'counter();                     \n' \
              'counter();                     \n'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ["1", "2"])

def test_should_print_native_function():
    source = 'print clock;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['<fn native>'])

def test_should_print_function_name():
    source = 'fun foo()                         \n' \
             '{                                 \n' \
             '    print 123;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'print foo;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['<fn foo>'])

def test_should_call_function_with_single_argument():
    source = 'fun foo(arg)                      \n' \
             '{                                 \n' \
             '    print arg;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'foo("bar");'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"bar"'])

def test_should_call_function_with_multiple_argument():
    source = 'fun foo(one, two)                 \n' \
             '{                                 \n' \
             '    print one;                    \n' \
             '    print two;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'foo("bar", "foobar");'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"bar"', '"foobar"'])

def test_should_call_function_with_sideeffect_argument():
    source = 'fun foo(arg)                      \n' \
             '{                                 \n' \
             '    print arg;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'foo(1 + 2);'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['3'])

def test_should_not_call_non_callable_type():
    source = '"not_a_function"();'

    result, _, stderr = rlox.run(source)
    assert rlox.failed(result, stderr)

def test_should_not_call_function_with_extra_args():
    source = 'fun foo(arg)                      \n' \
             '{                                 \n' \
             '    print arg;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'foo();'

    result, _, stderr = rlox.run(source)
    assert rlox.failed(result, stderr)

def test_should_not_call_function_with_missing_args():
    source = 'fun foo(arg)                      \n' \
             '{                                 \n' \
             '    print arg;                    \n' \
             '}                                 \n' \
             '                                  \n' \
             'foo(1, 2);'

    result, _, stderr = rlox.run(source)
    assert rlox.failed(result, stderr)