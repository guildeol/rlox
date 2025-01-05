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