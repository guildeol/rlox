import pytest

from tests.rlox import rlox

@pytest.mark.parametrize(
    'predicate,expected_output',
    [
        (
            'true and false',
            ['false']
        ),
        (
            'true and true',
            ['true']
        ),
        (
            'false and true',
            ['false']
        ),
        (
            'false and false',
            ['false']
        ),
    ]
)
def test_should_evaluate_and_expression(predicate, expected_output):
    source = f'var result = {predicate};\n'   \
              'print result;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, expected_output)

@pytest.mark.parametrize(
    'predicate,expected_output',
    [
        (
            'true or false',
            ['true']
        ),
        (
            'true or true',
            ['true']
        ),
        (
            'false or true',
            ['true']
        ),
        (
            'false or false',
            ['false']
        ),
    ]
)
def test_should_evaluate_or_expression(predicate, expected_output):
    source = f'var result = {predicate};\n'    \
              'print result;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, expected_output)