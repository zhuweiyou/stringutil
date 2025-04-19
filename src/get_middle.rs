pub fn get_middle(src: &str, left: &str, right: &str, more: bool) -> String {
    if src.is_empty() || left.is_empty() || right.is_empty() {
        return String::new();
    }

    let Some(left_start) = src.find(left) else {
        return String::new();
    };

    let search_start = left_start + left.len();
    let remaining = &src[search_start..];

    let right_pos = if more {
        remaining.rfind(right)
    } else {
        remaining.find(right)
    };

    let Some(right_start) = right_pos else {
        return String::new();
    };

    remaining[..right_start].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let s = "hello [world] example";
        assert_eq!(get_middle(s, "[", "]", false), "world");
        assert_eq!(get_middle(s, "[", "]", true), "world");
    }

    #[test]
    fn test_more_flag() {
        let s = "foo {bar} baz {qux}";
        assert_eq!(get_middle(s, "{", "}", false), "bar");
        assert_eq!(get_middle(s, "{", "}", true), "bar} baz {qux");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(get_middle("", "a", "b", false), "");
        assert_eq!(get_middle("text", "", "b", true), "");

        assert_eq!(get_middle("text", "#", "$", false), "");

        let s = "a <b> c <d> e </d> f </b> g";
        assert_eq!(get_middle(s, "<b>", "</b>", false), " c <d> e </d> f ");
        assert_eq!(get_middle(s, "<d>", "</d>", true), " e ");
    }

    #[test]
    fn test_unicode_support() {
        let s = "中文【测试】文本";
        assert_eq!(get_middle(s, "【", "】", false), "测试");

        let s2 = "🍎a🍊b🍌c🍎d🍌";
        assert_eq!(get_middle(s2, "🍊", "🍌", false), "b");
        assert_eq!(get_middle(s2, "🍎", "🍌", true), "a🍊b🍌c🍎d");
    }
}