pub fn encrypt(password: &str) -> String
{
    let characters = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'
    ];
    let mut encrypted_pasword = String::new();


    for char in password.chars()
    {
        for ch in 0..characters.len()
        {
            if char == characters[ch] //Couldn't think of a better way to do this :P
            {
                if ch+1 > characters.len()-1
                {
                    encrypted_pasword.push(characters[2]);
                }
                else if ch+2 > characters.len()-1 {
                    encrypted_pasword.push(characters[1]);
                }
                else if ch+3 > characters.len()-1 {
                    encrypted_pasword.push(characters[0]);
                }
                else {
                    encrypted_pasword.push(characters[ch+3]);
                }
            }
        }
    }

    return encrypted_pasword;
}
