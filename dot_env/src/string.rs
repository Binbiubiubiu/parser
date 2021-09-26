pub trait RemoveQuoted {
    fn remove_quoted(&self)->String;
}

impl RemoveQuoted for str{
    fn remove_quoted(&self)->String{

        let is_double_quoted = start_and_end_with(self,"\"");
        let is_single_quoted = start_and_end_with(self,"'");
        if is_double_quoted || is_single_quoted {
            let end = self.len()-1;
            let v = &self[1..end];
            if is_double_quoted {
                return v.replace("\\n", "\n");
            }
            return v.to_string();
        }
    
        self.to_string()
    }
}

#[inline]
fn start_and_end_with(val:&str,c:&str)->bool{
    if val.is_empty() {
        return false;
    }
    let end = val.len()-1;
    val.get(0..1) == Some(c) && val.get(end..) == Some(c)
}