#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::parse;
    use crate::state::NFA;

    const EMAIL_REGEX: &str = "[a-zA-Z][a-zA-Z0-9_.]+@[a-zA-Z0-9]+.[a-zA-Z]{2,}";

    #[test_case("valid_email@example.com", true; "plain address")]
    #[test_case("john.doe@email.com", true; "dot in local part")]
    #[test_case("user_name@email.org", true; "underscore in local part")]
    #[test_case("support@email.io", true; "two letter tld")]
    #[test_case("contact@123.com", true; "numeric domain")]
    #[test_case("sales@email.biz", true; "biz tld")]
    #[test_case("test_email@email.test", true; "test tld")]
    #[test_case("random.email@email.xyz", true; "xyz tld")]
    #[test_case("user@domain12345.com", true; "domain with trailing digits")]
    #[test_case("user@12345domain.com", true; "domain with leading digits")]
    // invalid when compared against our regex
    #[test_case("alice.smith123@email.co.uk", false; "multi label tld")]
    #[test_case("invalid.email@", false; "missing domain")]
    #[test_case(".invalid@email.com", false; "local part starts with dot")]
    #[test_case("email@invalid..com", false; "double dot in domain")]
    #[test_case("user@-invalid.com", false; "domain starts with hyphen")]
    #[test_case("user@invalid-.com", false; "domain ends with hyphen")]
    #[test_case("user@in valid.com", false; "space in domain")]
    #[test_case("user@.com", false; "empty domain before com")]
    #[test_case("user@.co", false; "empty domain before co")]
    #[test_case("user@domain.c", false; "single char tld")]
    #[test_case("user@domain.1a", false; "tld starts with digit")]
    #[test_case("user@domain.c0m", false; "digit inside tld")]
    #[test_case("user@domain..com", false; "double dot before tld")]
    #[test_case("user@.email.com", false; "leading dot in domain")]
    #[test_case("user@emai.l.com", false; "single char domain label")]
    #[test_case("user@e_mail.com", false; "underscore in domain")]
    #[test_case("user@e+mail.com", false; "plus in domain")]
    #[test_case("user@e^mail.com", false; "caret in domain")]
    #[test_case("user@e*mail.com", false; "asterisk in domain")]
    #[test_case("user@e.mail.com", false; "dot inside domain label")]
    #[test_case("user@e_mail.net", false; "underscore in domain net tld")]
    #[test_case("user@sub.domain.com", false; "subdomain")]
    #[test_case("user@sub-domain.com", false; "hyphenated domain")]
    #[test_case("user@sub.domain12345.com", false; "subdomain with digits")]
    #[test_case("user@sub.domain-12345.com", false; "subdomain with hyphen and digits")]
    #[test_case("user@-sub.domain.com", false; "subdomain starts with hyphen")]
    #[test_case("user@sub-.domain.com", false; "subdomain ends with hyphen")]
    #[test_case("user@domain-.com", false; "domain label ends with hyphen")]
    #[test_case("user@sub.domain.c0m", false; "subdomain with digit in tld")]
    #[test_case("user@sub.domain.c", false; "subdomain with single char tld")]
    #[test_case("user@sub.domain.1a", false; "subdomain with numeric tld")]
    #[test_case("user@sub.domain..com", false; "subdomain with double dot")]
    fn test_nfa(input: &str, expected: bool) {
        let ctx = parse(EMAIL_REGEX);
        let mut nfa = NFA { ..Default::default() };
        nfa.context_to_nfa(&ctx);

        assert_eq!(nfa.matches(input), expected, "input: {input}");
    }
}
