pub trait RemoveQuoted {
    fn remove_quoted(&self) -> String;
}

impl RemoveQuoted for str {
    fn remove_quoted(&self) -> String {
        let is_double_quoted = start_and_end_with(self, "\"", "\"");
        let is_single_quoted = start_and_end_with(self, "'", "'");

        let mut s = self.trim().to_string();

        if is_double_quoted || is_single_quoted {
            let end = s.len() - 1;
            let v = &s[1..end];

            s = v.to_string();

            if is_double_quoted {
                s =  s
                    .replace("\\\"", "\"")
                    .replace("\\r", "\r")
                    .replace("\\n", "\n");
            }
        }

        s.replace("\\#", "#").replace("\\;", ";")
    }
}

pub trait SplitDot {
    fn split_dot(&self) -> Vec<String>;
}

impl SplitDot for str {
    fn split_dot(&self) -> Vec<String> {
        self.replace("\\.", "____safe_dot____")
            .split(".")
            .map(|s| s.replace("____safe_dot____", ".").to_string())
            .collect()
    }
}

#[inline]
fn start_and_end_with(val: &str, s: &str, e: &str) -> bool {
    if val.is_empty() {
        return false;
    }
    let end = val.len() - 1;
    val.get(0..1) == Some(s) && val.get(end..) == Some(e)
}
