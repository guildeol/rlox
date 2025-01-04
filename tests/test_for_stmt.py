import pytest

from tests.rlox import rlox

def test_should_run_for_loop():
    source = 'for(var i = 0; i < 3; i = i + 1)  \n' \
             '{                                 \n' \
             '    print i;                      \n' \
             '}'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['0', '1', '2'])

def test_should_work_with_null_initializer():
    source = 'var i = 0;                        \n' \
             'for(; i < 3; i = i + 1)  \n' \
             '{                                 \n' \
             '    print i;                      \n' \
             '}'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['0', '1', '2'])

def test_should_work_with_null_increment():
    source = 'for(var i = 0; i < 3;)            \n' \
             '{                                 \n' \
             '    print i;                      \n' \
             '    i = i + 1;                    \n' \
             '}'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['0', '1', '2'])