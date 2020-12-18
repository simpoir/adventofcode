import ast


class Switcheroo(ast.NodeTransformer):
    def visit_Sub(self, node):
        return ast.Mult()

    def visit_Div(self, node):
        return ast.Add()


def day18(data):
    res1 = sum([
        eval(compile(Switcheroo().visit(ast.parse(
            line.replace('*', '-'), mode='eval'
        )), '', 'eval'))
        for line in data
    ])
    res2 = sum([
        eval(compile(Switcheroo().visit(ast.parse(
            line.translate({ord('*'): '-', ord('+'): '/'}),
            mode='eval'
        )), '', 'eval'))
        for line in data
    ])
    return res1, res2
