use std::fmt::Write;
use output::Output;

pub trait Tools: Output {
    /// Add programming-tools primitives.
    fn add_tools(&mut self) {
        self.add_primitive("words", Tools::words);
        self.add_primitive(".s", Tools::dot_s);
    }

    /// Run-time: ( -- )
    ///
    /// Display values on the data stack.
    primitive!{fn dot_s(&mut self) {
        let mut buf = self.output_buffer().take().unwrap();
        write!(buf, "TODO: .s").unwrap();
        //        write!(buf, "<{}> ", self.s_stack().len()).unwrap();
        //        for s in self.s_stack().iter() {
        //            write!(buf, "{} ", s).unwrap();
        //        }
        self.set_output_buffer(buf);
    }}

    /// Run-time: ( -- )
    ///
    /// List definition names in word list.
    primitive!{fn words(&mut self) {
        let mut buf = self.output_buffer().take().unwrap();
        writeln!(buf, "").unwrap();
        for w in self.wordlist().iter().rev() {
            let symbol = w.symbol();
            let hidden = w.is_hidden();
            if !hidden {
                write!(buf, "{} ", self.symbols()[symbol.id()]).unwrap();
            }
        }
        self.set_output_buffer(buf);
    }}
}
