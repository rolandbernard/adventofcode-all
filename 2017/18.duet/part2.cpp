
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
    RCV_R,
};

struct Instr {
    InstrOp op;
    int64_t args[2];
};

struct CpuState {
    size_t pc = 0;
    int64_t regs[26] = { 0 };
    queue<int64_t>& recv_buffer;
    queue<int64_t>& send_buffer;
    int64_t send_count = 0;
};

struct SystemState {
    CpuState cpu0;
    CpuState cpu1;
};

typedef vector<Instr> Program;

bool runCode(const Program& prog, CpuState& state) {
    bool advance = false;
    while (state.pc < prog.size()) {
        Instr instr = prog[state.pc];
        switch (instr.op) {
            case SND_I:
                state.send_buffer.push(instr.args[0]);
                state.send_count++;
                break;
            case SND_R:
                state.send_buffer.push(state.regs[instr.args[0]]);
                state.send_count++;
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
            case RCV_R:
                if (state.recv_buffer.empty()) {
                    return advance;
                } else {
                    state.regs[instr.args[0]] = state.recv_buffer.front();
                    state.recv_buffer.pop();
                }
                break;
        }
        state.pc++;
        advance = true;
    }
    return advance;
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
            instr.args[0] = s[0] - 'a';
            instr.op = RCV_R;
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

void runProgram(const Program& prog, SystemState& state) {
    while (runCode(prog, state.cpu0) || runCode(prog, state.cpu1));
}

int main() {
    Program prog = readProgram(cin);
    queue<int64_t> cpu0_queue;
    queue<int64_t> cpu1_queue;
    SystemState state = {
        .cpu0 = {.recv_buffer = cpu0_queue, .send_buffer = cpu1_queue},
        .cpu1 = {.recv_buffer = cpu1_queue, .send_buffer = cpu0_queue},
    };
    state.cpu1.regs['p' - 'a'] = 1;
    runProgram(prog, state);
    cout << "Result: " << state.cpu1.send_count << endl;
}

