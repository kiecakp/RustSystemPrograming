1. Zaprojektuj typ Towar (wraz z ewentualnymi typami pomocniczymi), który służy do przechowywania następujących cech towaru w pewnej hutowni:
   - opis/nazwa,
   - jednostka (sztuki, litry, kilogramy),
   - waga jednostkowa w kilogramach,
   - wymagane warunki przechowywania (zamrażarka, chłodziarka, normalne).
Konstruktor (ewentualnie konstruktory) tego typu nie powinien dopuszczać niedodatnich wag, a w przypadku jednostki ‘kilogramy’ — powinien wymuszać wagę jednostkową równą 1.0.

2. Zaprojektuj typ Zamowienie (wraz z ewentualnymi typami pomocniczymi), który przechowywać będzie listę Towarów (patrz zadanie poprzednie) wraz z ilością każdego. Potrzebne są następujące funkcje/metody:
   - kostruktor tworzący puste zamówienie;
   - metoda zwracająca sumaryczną wagę zamówienia;
   - metoda zwracająca sumaryczną wagę tych elementów zamówienia, które potrzebują zadanego sposobu przechowywania;
   - metoda (ewentualnie metody) dokładająca towar w podanej ilości do zamówienia, przy czym:
     - ilość musi być dodatnia;
     - dla jednostki ‘sztuki’ ilość musi być całkowita;
     - jeśli identyczny towar już jest w zamówieniu, to powinna być zwiększana jego ilość, a nie dokładany nowy element.
