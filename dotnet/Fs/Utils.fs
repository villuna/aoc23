module Utils

let split (delim: string) (str: string) = Seq.toList (str.Split(delim))