// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use qirlib::interop::{
    ClassicalRegister, Controlled, Instruction, Measured, QuantumRegister, Rotated, SemanticModel,
    Single,
};
use qirlib::{emit, interop};

use serial_test::serial;
use std::path::Path;
use std::{env, fs};
use std::{
    io::{self, Write},
    path::PathBuf,
};
use tempfile::tempdir;

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn zero_to_one_x_measure() {
    execute(
        "zero_to_one_x_measure",
        write_zero_to_one_x_measure,
        vec!["[[One]]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn zero_to_one_or_zero_h_measure() {
    execute(
        "zero_to_one_or_zero_h_measure",
        write_zero_to_one_or_zero_h_measure,
        vec!["[[Zero]]", "[[One]]"],
    );
}

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn one_to_one_or_zero_h_measure() {
    execute(
        "one_to_one_or_zero_h_measure",
        write_one_to_one_or_zero_h_measure,
        vec!["[[Zero]]", "[[One]]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn bell_circuit_with_measurement() {
    execute(
        "bell_measure",
        write_bell_measure,
        vec!["[[Zero, Zero]]", "[[One, One]]"],
    );
}

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn bell_circuit_no_measurement() {
    execute("bell_no_measure", write_bell_no_measure, vec!["[]"]);
}

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn empty_model() {
    execute("empty", write_empty_model, vec!["[]"]);
}

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn model_with_only_qubit_allocations() {
    execute(
        "model_with_only_qubit_allocations",
        write_model_with_only_qubit_allocations,
        vec!["[]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn model_with_only_result_allocations() {
    execute(
        "model_with_only_result_allocations",
        write_model_with_only_result_allocations,
        vec!["[[Zero, Zero, Zero, Zero], [Zero, Zero, Zero], [Zero, Zero]]"],
    );
}

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn model_with_no_instructions() {
    execute(
        "model_with_no_instructions",
        write_model_with_no_instructions,
        vec!["[[Zero, Zero]]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn model_with_single_qubit_instructions() {
    execute(
        "model_with_single_qubit_instructions",
        write_model_with_single_qubit_instructions,
        vec!["[]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn single_qubit_model_with_measurement() {
    execute(
        "single_qubit_model_with_measurement",
        write_single_qubit_model_with_measurement,
        vec!["[[Zero]]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn model_with_instruction_cx() {
    execute(
        "model_with_instruction_cx",
        write_model_with_instruction_cx,
        vec!["[]"],
    );
}

#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn model_with_instruction_cz() {
    execute(
        "model_with_instruction_cz",
        write_model_with_instruction_cz,
        vec!["[]"],
    );
}
#[test]
#[serial]
#[ignore = "TODO: Replace with JIT execution."]
fn bernstein_vazirani() {
    execute(
        "bernstein_vazirani",
        write_bernstein_vazirani,
        vec!["[[Zero, One, Zero, One, One]]"],
    );
}

// Assumes compiler is on the path.
fn get_compiler_name() -> &'static str {
    if cfg!(target_os = "windows") {
        return r"clang++";
    } else {
        return r"clang++-11";
    }
}
fn execute(name: &str, generator: fn(&str) -> (), expected_results: Vec<&str>) {
    let dir = tempdir().expect("Could not create temporary directory");
    let test_dir = dir.path().to_path_buf();
    let ir_path = dir.path().join(format!("{}.ll", name.replace(" ", "_")));

    // generate the QIR
    log::debug!("Writing {:?}", ir_path);
    generator(ir_path.display().to_string().as_str());

    let mut app = dir.path().join(name);

    // todo change extension based on platform
    // ie exe on windows.
    if cfg!(target_os = "windows") {
        // add .exe
        app.set_extension("exe");
    }

    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR missing")
            .to_str()
            .unwrap(),
    );
    let native = PathBuf::from(
        env::var_os("QIR_RUNTIME_LIB")
            .expect("QIR_RUNTIME_LIB missing")
            .to_str()
            .unwrap(),
    );

    let include = PathBuf::from(
        env::var_os("QIR_RUNTIME_INCLUDE")
            .expect("QIR_RUNTIME_INCLUDE missing")
            .to_str()
            .unwrap(),
    );

    let simulators_native = PathBuf::from(
        env::var_os("QSHARP_NATIVE_SIMULATORS")
            .expect("QSHARP_NATIVE_SIMULATORS missing")
            .to_str()
            .unwrap(),
    );

    println!("{}", native.display());
    println!("{}", include.display());
    println!("{}", simulators_native.display());

    copy_files(&native, &test_dir);
    copy_files(&include, &test_dir);
    copy_files(&simulators_native, &test_dir);

    let mut main_cpp = PathBuf::from(&manifest_dir);
    main_cpp.push("tests");
    main_cpp.push("main.cpp");
    let compiler = get_compiler_name();
    let mut command = std::process::Command::new(compiler);
    command
        .arg("-o")
        .arg(app.to_str().unwrap())
        //.arg("-S")
        //.arg("-emit-llvm")
        .arg(ir_path)
        .arg(main_cpp)
        .arg(format!("-I{}", include.to_str().unwrap()))
        .arg(format!("-L{}", native.to_str().unwrap()))
        .args([
            "-lMicrosoft.Quantum.Qir.Runtime",
            "-lMicrosoft.Quantum.Qir.QSharp.Core",
            "-lMicrosoft.Quantum.Qir.QSharp.Foundation",
        ]);

    if cfg!(target_os = "windows") {
        command
            .arg("-target")
            .arg("x86_64-pc-windows-msvc19.29.30038");
    }

    println!("{:?}", command);
    let output = command.output().expect("failed to execute process");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    execute_circuit(app.to_str().unwrap(), expected_results);
}

fn copy_files(source: &PathBuf, target: &PathBuf) {
    if let Ok(entries) = fs::read_dir(source) {
        for path in entries {
            let file_name = path.unwrap().path();
            if file_name.is_file() {
                let file = file_name.file_name().unwrap().to_str().unwrap();
                let src = format!("{}/{}", source.display(), file);
                let dst = format!("{}/{}", target.display(), file);

                std::fs::copy(&src, &dst).expect(
                    format!("Failed to copy {} to {}", src.as_str(), dst.as_str()).as_str(),
                );
            }
        }
    }
}

fn execute_circuit(app: &str, expected_results: Vec<&str>) {
    let parent = String::from(Path::new(app).parent().unwrap().to_str().unwrap());

    let mut command = std::process::Command::new(app);
    if cfg!(target_os = "linux") {
        if let Ok(existing_value) = env::var("LD_LIBRARY_PATH") {
            let ld_path = format!("{}:{}", parent.as_str(), existing_value);
            command.env("LD_LIBRARY_PATH", ld_path);
        } else {
            command.env("LD_LIBRARY_PATH", parent);
        }
    }

    println!("{:?}", command);
    let output = command.output().expect("failed to execute process");

    println!("status: {}", output.status);
    let stdout = String::from_utf8(output.stdout).unwrap().trim().to_owned();
    let stderr = String::from_utf8(output.stderr).unwrap().trim().to_owned();

    println!("out: {}", stdout.as_str());
    eprintln!("err: {}", stderr.as_str());

    assert!(output.status.success());
    assert!(expected_results.iter().any(|&x| x == stdout.as_str()));
}

fn write_empty_model(file_name: &str) {
    let name = String::from("empty");
    let model = SemanticModel::new(name);
    interop::emit::write(&model, file_name).unwrap();
}
fn write_model_with_single_qubit_instructions(file_name: &str) {
    let name = String::from("model_with_single_qubit_instructions");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());

    model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::Reset(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::Rx(Rotated::new(15.0, String::from("qr0"))));
    model.add_inst(Instruction::Ry(Rotated::new(16.0, String::from("qr0"))));
    model.add_inst(Instruction::Rz(Rotated::new(17.0, String::from("qr0"))));
    model.add_inst(Instruction::S(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::SAdj(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::T(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::TAdj(Single::new(String::from("qr0"))));

    interop::emit::write(&model, file_name).unwrap();
}
fn write_model_with_instruction_cx(file_name: &str) {
    let name = String::from("model_with_instruction_cx");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());

    model.add_inst(Instruction::Cx(Controlled::new(
        String::from("qr0"),
        String::from("qr1"),
    )));

    interop::emit::write(&model, file_name).unwrap();
}

fn write_model_with_instruction_cz(file_name: &str) {
    let name = String::from("model_with_instruction_cz");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());

    model.add_inst(Instruction::Cz(Controlled::new(
        String::from("qr0"),
        String::from("qr1"),
    )));

    interop::emit::write(&model, file_name).unwrap();
}
fn write_model_with_only_qubit_allocations(file_name: &str) {
    let name = String::from("model_with_only_qubit_allocations");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());
    interop::emit::write(&model, file_name).unwrap();
}
fn write_model_with_only_result_allocations(file_name: &str) {
    let name = String::from("model_with_only_result_allocations");
    let mut model = SemanticModel::new(name);
    model.add_reg(ClassicalRegister::new(String::from("qa"), 4).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qb"), 3).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 2).as_register());
    interop::emit::write(&model, file_name).unwrap();
}
fn write_model_with_no_instructions(file_name: &str) {
    let name = String::from("model_with_no_instructions");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 2).as_register());
    interop::emit::write(&model, file_name).unwrap();
}
fn write_bell_no_measure(file_name: &str) {
    let name = String::from("Bell circuit");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());

    model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::Cx(Controlled::new(
        String::from("qr0"),
        String::from("qr1"),
    )));
    interop::emit::write(&model, file_name).unwrap();
}

fn write_single_qubit_model_with_measurement(file_name: &str) {
    let name = String::from("single_qubit_model_with_measurement");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 1).as_register());

    model.add_inst(Instruction::M(Measured::new(
        String::from("qr0"),
        String::from("qc0"),
    )));

    interop::emit::write(&model, file_name).unwrap();
}
fn write_one_to_one_or_zero_h_measure(file_name: &str) {
    let name = String::from("write_one_to_one_or_zero_h_measure");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 1).as_register());

    model.add_inst(Instruction::X(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::M(Measured::new(
        String::from("qr0"),
        String::from("qc0"),
    )));
    interop::emit::write(&model, file_name).unwrap();
}
fn write_zero_to_one_or_zero_h_measure(file_name: &str) {
    let name = String::from("write_zero_to_one_or_zero_h_measure");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 1).as_register());

    model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::M(Measured::new(
        String::from("qr0"),
        String::from("qc0"),
    )));
    interop::emit::write(&model, file_name).unwrap();
}
fn write_zero_to_one_x_measure(file_name: &str) {
    let name = String::from("Bell circuit");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 1).as_register());

    model.add_inst(Instruction::X(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::M(Measured::new(
        String::from("qr0"),
        String::from("qc0"),
    )));
    interop::emit::write(&model, file_name).unwrap();
}
fn write_bell_measure(file_name: &str) {
    let name = String::from("Bell circuit");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("qr"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("qr"), 1).as_register());
    model.add_reg(ClassicalRegister::new(String::from("qc"), 2).as_register());

    model.add_inst(Instruction::H(Single::new(String::from("qr0"))));
    model.add_inst(Instruction::Cx(Controlled::new(
        String::from("qr0"),
        String::from("qr1"),
    )));
    model.add_inst(Instruction::M(Measured::new(
        String::from("qr0"),
        String::from("qc0"),
    )));
    model.add_inst(Instruction::M(Measured::new(
        String::from("qr1"),
        String::from("qc1"),
    )));
    interop::emit::write(&model, file_name).unwrap();
}

fn write_bernstein_vazirani(file_name: &str) {
    let name = String::from("Bernstein-Vazirani circuit");
    let mut model = SemanticModel::new(name);
    model.add_reg(QuantumRegister::new(String::from("input_"), 0).as_register());
    model.add_reg(QuantumRegister::new(String::from("input_"), 1).as_register());
    model.add_reg(QuantumRegister::new(String::from("input_"), 2).as_register());
    model.add_reg(QuantumRegister::new(String::from("input_"), 3).as_register());
    model.add_reg(QuantumRegister::new(String::from("input_"), 4).as_register());

    model.add_reg(QuantumRegister::new(String::from("target_"), 0).as_register());

    model.add_reg(ClassicalRegister::new(String::from("output_"), 5).as_register());

    model.add_inst(Instruction::X(Single::new(String::from("target_0"))));

    model.add_inst(Instruction::H(Single::new(String::from("input_0"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_1"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_2"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_3"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_4"))));
    model.add_inst(Instruction::H(Single::new(String::from("target_0"))));

    // random chosen
    model.add_inst(Instruction::Cx(Controlled::new(
        String::from("input_1"),
        String::from("target_0"),
    )));
    model.add_inst(Instruction::Cx(Controlled::new(
        String::from("input_3"),
        String::from("target_0"),
    )));
    model.add_inst(Instruction::Cx(Controlled::new(
        String::from("input_4"),
        String::from("target_0"),
    )));
    model.add_inst(Instruction::H(Single::new(String::from("input_0"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_1"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_2"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_3"))));
    model.add_inst(Instruction::H(Single::new(String::from("input_4"))));

    model.add_inst(Instruction::M(Measured::new(
        String::from("input_0"),
        String::from("output_0"),
    )));
    model.add_inst(Instruction::M(Measured::new(
        String::from("input_1"),
        String::from("output_1"),
    )));
    model.add_inst(Instruction::M(Measured::new(
        String::from("input_2"),
        String::from("output_2"),
    )));
    model.add_inst(Instruction::M(Measured::new(
        String::from("input_3"),
        String::from("output_3"),
    )));
    model.add_inst(Instruction::M(Measured::new(
        String::from("input_4"),
        String::from("output_4"),
    )));

    model.add_inst(Instruction::Reset(Single::new(String::from("input_0"))));
    model.add_inst(Instruction::Reset(Single::new(String::from("input_1"))));
    model.add_inst(Instruction::Reset(Single::new(String::from("input_2"))));
    model.add_inst(Instruction::Reset(Single::new(String::from("input_3"))));
    model.add_inst(Instruction::Reset(Single::new(String::from("input_4"))));

    interop::emit::write(&model, file_name).unwrap();
}
