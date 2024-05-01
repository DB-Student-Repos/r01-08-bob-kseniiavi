pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() || message.is_empty() {
        return "Fine. Be that way!";
    }

    if message.contains("?") && message.chars().find(|c| c.is_alphabetic()).is_some() && message.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        return "Calm down, I know what I'm doing!";
    } 

    if let Some(index) = message.find('?') {
            if message[index + 1..].trim().is_empty() {
                return "Sure.";
            }
        }

    if message.chars().find(|c| c.is_alphabetic()).is_some() && message.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        return "Whoa, chill out!";
    }

    "Whatever."

    //unimplemented!("have Bob reply to the incoming message: {message}")
}
