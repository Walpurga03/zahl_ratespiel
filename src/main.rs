// Importiere das Modul für Eingabe und Ausgabe
use std::io;

// Importiere die Funktionalitäten des Zufallsgenerators aus dem `rand`-Crate
use rand::Rng;

// Importiere das `Ordering`-Enum für Vergleichsergebnisse
use std::cmp::Ordering;

fn main() {
    // Drucke eine Begrüßungsnachricht auf die Konsole
    println!("Errate eine Zahl von 1 bis 100!");

    // Erzeuge eine Zufallszahl zwischen 1 und 100 (inklusive) und speichere sie in `secret_number`
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Eine Endlosschleife, damit der Benutzer immer wieder raten kann, bis er gewinnt
    loop {
        // Drucke eine Nachricht, die den Benutzer auffordert, eine Zahl einzugeben
        println!("Bitte gib deinen Tipp ein.");

        // Erzeuge eine neue, leere String-Variable, um die Benutzereingabe zu speichern
        let mut guess = String::new();

        // Lies die Benutzereingabe von der Standard-Eingabe (Konsole) in die Variable `guess`
        io::stdin()
            .read_line(&mut guess)
            .expect("Fehler beim Lesen der Eingabe."); // Gebe eine Fehlermeldung aus, falls das Lesen fehlschlägt

        // Versuche, die Benutzereingabe in eine Ganzzahl (`u32`) zu parsen
        let guess: u32 = match guess.trim().parse() {
            // Falls das Parsen erfolgreich ist, speichere den Wert in `num`
            Ok(num) => num,
            // Falls das Parsen fehlschlägt (z.B. der Benutzer gibt keine Zahl ein), überspringe den Rest der Schleife und fordere erneut zur Eingabe auf
            Err(_) => continue,
        };

        // Drucke die geratene Zahl auf die Konsole
        println!("Du hast getippt: {guess}");

        // Vergleiche die geratene Zahl (`guess`) mit der geheimen Zahl (`secret_number`)
        match guess.cmp(&secret_number) {
            // Falls die geratene Zahl kleiner ist, drucke "Too small!" auf die Konsole
            Ordering::Less => println!("Zu klein!"),
            // Falls die geratene Zahl größer ist, drucke "Too big!" auf die Konsole
            Ordering::Greater => println!("Zu groß!"),
            // Falls die geratene Zahl gleich ist, drucke "You win!" auf die Konsole und beende die Schleife
            Ordering::Equal => {
                println!("Du hast gewonnen!");
                break; // Beende die Schleife, da der Benutzer die Zahl richtig geraten hat
            }
        }
    }
}

