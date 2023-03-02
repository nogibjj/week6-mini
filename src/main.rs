//build a RUST project with a beverate menu app

//import the io library
use std::io;

//define the main function

fn main() {
    //print a welcome message
    println!("Welcome to the beverage menu app");
    //print a menu
    println!("1. Coffee");
    println!("2. Tea");
    println!("3. Soda");
    println!("4. Water");
    println!("5. Milk");
    println!("6. Juice");
    println!("7. Beer");
    println!("8. Wine");
    println!("9. Whiskey");
    println!("10. Vodka");
    println!("11. Rum");
    println!("12. Gin");
    println!("13. Brandy");
    println!("14. Champagne");
    println!("15. Cognac");
    println!("16. Liqueur");
    println!("17. Cordial");
    println!("18. Sherry");
    println!("19. Vermouth");
    println!("20. Mead");
    println!("21. Cider");
    println!("22. Ale");
    println!("23. Stout");
    println!("24. Lager");
    println!("25. Pilsner");
    println!("26. Porter");
    println!("27. Sake");
    println!("28. Shochu");
    println!("29. Soju");
    println!("30. Rice Wine");
    println!("31. Beer Wine");
    println!("32. Beer Shochu");
    println!("33. Beer Soju");
    println!("34. Beer Rice Wine");
    println!("35. Shochu Soju");
    println!("36. Shochu Rice Wine");
    println!("37. Soju Rice Wine");
    println!("38. Shochu Soju Rice Wine");
    println!("39. Beer Shochu Soju");
    println!("40. Beer Shochu Rice Wine");
    println!("41. Beer Soju Rice Wine");
    println!("42. Shochu Soju Rice Wine");
    println!("43. Beer Shochu Soju Rice Wine");
    println!("44. Beer Wine Shochu");
    println!("45. Beer Wine Soju");
    println!("46. Beer Wine Rice Wine");
    println!("47. Beer Wine Shochu Soju");
    println!("48. Beer Wine Shochu Rice Wine");
    println!("49. Beer Wine Soju Rice Wine");
    println!("50. Beer Wine Shochu Soju Rice Wine");
    println!("51. Wine Shochu");


    //print a prompt
    println!("Please enter a number to select a beverage");

    //declare a variable to hold the user input
    let mut user_input = String::new();

    //read the user input
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    //convert the user input to a number
    let user_input: u32 = user_input.trim().parse()
        .expect("Please type a number!");

    //print the user input
    println!("You selected {}", user_input);

    //print a goodbye message
    println!("Thank you for using the beverage menu app");

}

