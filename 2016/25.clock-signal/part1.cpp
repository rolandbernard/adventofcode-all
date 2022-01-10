
#include <bits/stdc++.h>

using namespace std;

enum InstrOp {
    INC_R,
    DEC_R,
    CPY_RR,
    CPY_IR,
    JNZ_II,
    JNZ_IR,
    JNZ_RI,
    JNZ_RR,
    OUT_I,
    OUT_R,
};

struct Instr {
    InstrOp op;
    int64_t args[2];
};

struct CpuState {
    size_t pc = 0;
    int64_t regs[4] = {0, 0, 0, 0};
};

typedef vector<Instr> Program;

void runProgram(const Program& prog, CpuState& state, function<bool (int64_t)> out) {
    while (state.pc < prog.size()) {
        Instr instr = prog[state.pc];
        state.pc++;
        switch (instr.op) {
            case INC_R:
                state.regs[instr.args[0]]++;
                break;
            case DEC_R:
                state.regs[instr.args[0]]--;
                break;
            case CPY_RR:
                state.regs[instr.args[1]] = state.regs[instr.args[0]];
                break;
            case CPY_IR:
                state.regs[instr.args[1]] = instr.args[0];
                break;
            case JNZ_II:
                if (instr.args[0] != 0) {
                    state.pc += instr.args[1] - 1;
                }
                break;
            case JNZ_IR:
                if (instr.args[0] != 0) {
                    state.pc += state.regs[instr.args[1]] - 1;
                }
                break;
            case JNZ_RI:
                if (state.regs[instr.args[0]] != 0) {
                    state.pc += instr.args[1] - 1;
                }
                break;
            case JNZ_RR:
                if (state.regs[instr.args[0]] != 0) {
                    state.pc += state.regs[instr.args[1]] - 1;
                }
                break;
            case OUT_I:
                if (!out(instr.args[0])) {
                    return;
                }
                break;
            case OUT_R:
                if (!out(state.regs[instr.args[0]])) {
                    return;
                }
                break;
        }
    }
}

Program readProgram(istream& in) {
    Program p;
    string s;
    while (in >> s) {
        Instr instr;
        if (s == "cpy") {
            in >> s;
            if (isalpha(s[0])) {
                instr.args[0] = s[0] - 'a';
                instr.op = CPY_RR;
            } else {
                instr.args[0] = stol(s);
                instr.op = CPY_IR;
            }
            in >> s;
            instr.args[1] = s[0] - 'a';
        } else if (s == "jnz") {
            in >> s;
            if (isalpha(s[0])) {
                instr.args[0] = s[0] - 'a';
                in >> s;
                if (isalpha(s[0])) {
                    instr.args[1] = s[0] - 'a';
                    instr.op = JNZ_RR;
                } else {
                    instr.args[1] = stol(s);
                    instr.op = JNZ_RI;
                }
            } else {
                instr.args[0] = stol(s);
                in >> s;
                if (isalpha(s[0])) {
                    instr.args[1] = s[0] - 'a';
                    instr.op = JNZ_IR;
                } else {
                    instr.args[1] = stol(s);
                    instr.op = JNZ_II;
                }
            }
        } else if (s == "inc") {
            in >> s;
            instr.args[0] = s[0] - 'a';
            instr.op = INC_R;
        } else if (s == "dec") {
            in >> s;
            instr.args[0] = s[0] - 'a';
            instr.op = DEC_R;
        } else if (s == "out") {
            in >> s;
            if (isalpha(s[0])) {
                instr.args[0] = s[0] - 'a';
                instr.op = OUT_R;
            } else {
                instr.args[0] = stol(s);
                instr.op = OUT_I;
            }
        }
        p.push_back(instr);
    }
    return p;
}

int main() {
    Program prog = readProgram(cin);
    size_t i;
    for (i = 0; ; i++) {
        CpuState state;
        state.regs[0] = i;
        bool valid = true;
        auto out = [&valid, j = 0] (int64_t i) mutable {
            if (j < 100 && i == j % 2) {
                j++;
            } else if (j >= 100) {
                return false;
            } else {
                valid = false;
                return false;
            }
            return true;
        };
        runProgram(prog, state, out);
        if (valid) {
            break;
        }
    }
    cout << "Result: " << i << endl;
}

