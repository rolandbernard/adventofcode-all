
#include <bits/stdc++.h>

using namespace std;

enum InstrOp {
    SND_I,
    SND_R,
    JGZ_II,
    JGZ_IR,
    JGZ_RI,
    JGZ_RR,
    SET_RI,
    SET_RR,
    ADD_RI,
    ADD_RR,
    MUL_RI,
    MUL_RR,
    REM_RI,
    REM_RR,
    RCV_I,
    RCV_R,
};

struct Instr {
    InstrOp op;
    int64_t args[2];
};

struct CpuState {
    size_t pc = 0;
    int64_t send = 0;
    int64_t regs[26] = {0};
};

typedef vector<Instr> Program;

void runCode(const Program& prog, CpuState& state) {
    while (state.pc < prog.size()) {
        Instr instr = prog[state.pc];
        state.pc++;
        switch (instr.op) {
            case SND_I:
                state.send = instr.args[0];
                break;
            case SND_R:
                state.send = state.regs[instr.args[0]];
                break;
            case JGZ_II:
                if (instr.args[0] > 0) {
                    state.pc += instr.args[1] - 1;
                }
                break;
            case JGZ_IR:
                if (instr.args[0] > 0) {
                    state.pc += state.regs[instr.args[1]] - 1;
                }
                break;
            case JGZ_RI:
                if (state.regs[instr.args[0]] > 0) {
                    state.pc += instr.args[1] - 1;
                }
                break;
            case JGZ_RR:
                if (state.regs[instr.args[0]] > 0) {
                    state.pc += state.regs[instr.args[1]] - 1;
                }
                break;
            case SET_RI:
                state.regs[instr.args[0]] = instr.args[1];
                break;
            case SET_RR:
                state.regs[instr.args[0]] = state.regs[instr.args[1]];
                break;
            case ADD_RI:
                state.regs[instr.args[0]] += instr.args[1];
                break;
            case ADD_RR:
                state.regs[instr.args[0]] += state.regs[instr.args[1]];
                break;
            case MUL_RI:
                state.regs[instr.args[0]] *= instr.args[1];
                break;
            case MUL_RR:
                state.regs[instr.args[0]] *= state.regs[instr.args[1]];
                break;
            case REM_RI:
                state.regs[instr.args[0]] %= instr.args[1];
                break;
            case REM_RR:
                state.regs[instr.args[0]] %= state.regs[instr.args[1]];
                break;
            case RCV_I:
                if (instr.args[0] != 0) {
                    return;
                }
                break;
            case RCV_R:
                if (state.regs[instr.args[0]] != 0) {
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
        if (s == "snd") {
            in >> s;
            if (isalpha(s[0])) {
                instr.args[0] = s[0] - 'a';
                instr.op = SND_R;
            } else {
                instr.args[0] = stol(s);
                instr.op = SND_I;
            }
        } else if (s == "rcv") {
            in >> s;
            if (isalpha(s[0])) {
                instr.args[0] = s[0] - 'a';
                instr.op = RCV_R;
            } else {
                instr.args[0] = stol(s);
                instr.op = RCV_I;
            }
        } else if (s == "jgz") {
            in >> s;
            if (isalpha(s[0])) {
                instr.args[0] = s[0] - 'a';
                in >> s;
                if (isalpha(s[0])) {
                    instr.args[1] = s[0] - 'a';
                    instr.op = JGZ_RR;
                } else {
                    instr.args[1] = stol(s);
                    instr.op = JGZ_RI;
                }
            } else {
                instr.args[0] = stol(s);
                in >> s;
                if (isalpha(s[0])) {
                    instr.args[1] = s[0] - 'a';
                    instr.op = JGZ_IR;
                } else {
                    instr.args[1] = stol(s);
                    instr.op = JGZ_II;
                }
            }
        } else if (s == "set") {
            in >> s;
            instr.args[0] = s[0] - 'a';
            in >> s;
            if (isalpha(s[0])) {
                instr.args[1] = s[0] - 'a';
                instr.op = SET_RR;
            } else {
                instr.args[1] = stol(s);
                instr.op = SET_RI;
            }
        } else if (s == "add") {
            in >> s;
            instr.args[0] = s[0] - 'a';
            in >> s;
            if (isalpha(s[0])) {
                instr.args[1] = s[0] - 'a';
                instr.op = ADD_RR;
            } else {
                instr.args[1] = stol(s);
                instr.op = ADD_RI;
            }
        } else if (s == "mul") {
            in >> s;
            instr.args[0] = s[0] - 'a';
            in >> s;
            if (isalpha(s[0])) {
                instr.args[1] = s[0] - 'a';
                instr.op = MUL_RR;
            } else {
                instr.args[1] = stol(s);
                instr.op = MUL_RI;
            }
        } else if (s == "mod") {
            in >> s;
            instr.args[0] = s[0] - 'a';
            in >> s;
            if (isalpha(s[0])) {
                instr.args[1] = s[0] - 'a';
                instr.op = REM_RR;
            } else {
                instr.args[1] = stol(s);
                instr.op = REM_RI;
            }
        } else {
            cerr << "ERROR: " << s << endl;
        }
        p.push_back(instr);
    }
    return p;
}

int main() {
    Program prog = readProgram(cin);
    CpuState state;
    runCode(prog, state);
    cout << "Result: " << state.send << endl;
}

