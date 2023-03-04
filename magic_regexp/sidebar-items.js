window.SIDEBAR_ITEMS = {"enum":[["Error","An error, which can occur while using this crate. Mostly used to wrap errors from the Regex crate."],["Input","This is a regex input that can be used to match a single character or a group of characters. Can be used to create a regex that matches a single character or a group of characters. For example, `Input::Exactly(Type::Digit)` will match a single digit."],["Type","Represents a regex type. This enum is used to create the smallest regex statement. For example, `Type::Digit` will create the regex `\\d`."]],"fn":[["create_reg_exp","Returns the regex, which represents the given statement. This is only for convenience and compatibility with magic-regex from npm."],["not","Returns the opposite of the given type. For example, `Type::Digit` will return `Type::NotDigit`. Returns the same type if it is not a type that can be negated."]],"trait":[["AsRegex","A trait, which allows to convert something to a regex. Mostly needed to work with this lib and Regex crate."],["Condition","A trait, which allows to chain regex statements with conditions. Import this, if you want to use the `and`, `or` and `optionally` methods and chain statements."]],"type":[["Result","A type, which is used to return results from this crate. Mostly used to wrap results from the Regex crate."]]};