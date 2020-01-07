use yew::prelude::*;
use crate::umlaut_replacer::replace_umlaute;

pub struct Model {
    link: ComponentLink<Self>,
    entered_text: String,
    edited_text: String,
    ignore_words: Vec<String>
}

pub enum Msg {
    SetText(String)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ignore_words = vec![
            "neue", "Frauen", "Michael", "Zuschauer", "Neue", "Quellen", "Feuerwehr", "Quelle",
            "Israel", "aktuellen", "Feuer", "zuerst", "aktuelle", "freuen",
            "bauen", "Vertrauen", "Bauern", "League", "Steuern", "Bauer",
            "heuer", "Dauer", "aktuell", "dauert", "teuer", "Konsequenzen",
            "schauen", "Duell", "Mauer", "Manuel", "genauer", "Steuer", "israelischen",
            "genaue", "eventuell", "Abenteuer", "freue", "konsequent",
            "dauern", "neuesten", "israelische", "quer", "Blues", "Jacques",
            "Zuerst", "Konsequenz", "blauen", "Joe", "individuelle", "dauerhaft",
            "Prenzlauer", "Bildhauer", "Aktuell", "Samuel", "Erneuerung",
            "Steuerzahler", "Goethe", "erneuert", "Blue", "Trauer", "Ueckermünde",
            "aufzubauen", "ausbauen", "Betreuer", "Feuerwehren", "aufbauen",
            "sexuellen", "Selbstvertrauen", "Queen", "Moskauer", "Neuenahr",
            "soeben", "Internetquelle", "blaue", "Hoeneß", "Boeing", "Thurgauer",
            "neuerdings", "individuell", "Treue", "Steuerreform", "Feuerwerk",
            "bedauert", "Brauerei", "anschauen", "Buenos", "sexuelle",
            "Litauen", "Poesie", "Mehrwertsteuer", "grauen", "Feuerwehrleute",
            "Israelis", "auszubauen", "Frauenfeld", "Neuerungen", "Rafael",
            "sauer", "Revue", "Bauen", "steuert", "steuern", "Steuereinnahmen",
            "neueste", "betreuen", "bequem", "tue", "Bauernhof", "Auer",
            "gedauert", "Feuerwehrhaus", "Venezuela", "Beckenbauer",
            "erneuerbaren", "abbauen", "Phoenix", "sexuell", "trauen",
            "Steuerung", "abzubauen", "Landfrauen", "Autobauer", "Frauenchor",
            "Raphael", "Misstrauen", "neueren", "zueinander", "Sauer",
            "Sauerstoff", "Steuerhinterziehung", "vertrauen", "Gewerbesteuer",
            "Dominique", "einbauen", "Breuer", "gesteuert", "Besteuerung",
            "Schauer", "Statue", "Blaue", "virtuellen", "dauernden", "erfreuen",
            "Miguel", "Warschauer", "Adenauer", "Untreue", "Individuen",
            "neuerlichen", "Jugendfeuerwehr", "Oettinger", "fuer", "graue",
            "Ruedi", "Emanuel", "virtuelle", "erneuerbare", "schaue", "Ueli",
            "Aue", "Uecker-Randow", "Frequenz", "Ramsauer", "Intellektuellen",
            "Avenue", "Ausdauer", "Querschnitt", "Neuerung", "Trauerfeier",
            "Quellenangabe", "Steuersenkungen", "dauerhaften", "andauernden",
            "neuere", "Grauen", "Leuenberger", "Bedauern", "Steuererhöhungen",
            "Feuerwehrmann", "Ökosteuer", "steuerliche", "Querelen",
            "Frauengemeinschaft", "Lauer", "treue", "Puerto", "erneuern",
            "neuerlich", "internetquelle", "Caesar", "überqueren", "bedauerlich",
            "Jacqueline", "Dorferneuerung", "Lagerfeuer", "Sauerland",
            "steuerlich", "Feuerwache", "anzuerkennen", "teuerste", "anzuschauen",
            "Nassauer", "Einkommensteuer", "Reue", "Frequenzen", "euer",
            "Lebensdauer", "Brauer", "intellektuellen", "Gusenbauer",
            "Duett", "Heuer", "Goebbels", "Berufsfeuerwehr", "Dauerausstellung",
            "Neuendorf", "eventuellen", "sueddeutsche", "dauernde", "Neuenburg",
            "Rodriguez", "Neubauer", "zuschauen", "beteuerte", "Intellektuelle",
            "Olympique", "intellektuelle", "Hanauer", "Steuergeldern",
            "Bauernmarkt", "Eventuell", "Steuerberater", "Hoechst", "einzubauen",
            "Feuerwehrmänner", "Aktuellen", "dauerten", "Auerbach", "Hauer",
            "beteuert", "aufrechtzuerhalten", "andauernde", "Teuerung",
            "Hofbauer", "Reggae", "Schaefer", "bäuerlichen", "gefeuert",
            "Frauenbewegung", "Euer", "Bauernverband", "ungeheuer", "poetischen",
            "Steuerpolitik", "beisteuern", "Stadtmauer", "Rue", "trauert",
            "Steuerfuss", "Landauer", "Neuenahr-Ahrweiler", "Brauereien",
            "Hauenstein", "Mauerfall", "Ehefrauen", "poetische", "Steuerrecht",
            "Feuerwehrgerätehaus", "Mauerwerk", "Grundsteuer", "Steuergelder",
            "Passauer", "Mißtrauen", "Monroe", "Oboe", "Bellevue", "traue",
            "scheuen", "Frauenhaus", "Emmanuel", "bedauern", "Schauen",
            "abgefeuert", "Homosexuelle", "manuell", "Michaelis", "überquerte",
            "Querflöte", "umbauen", "Hausfrauen", "Steuererklärung",
            "konsequenten", "homosexuelle", "besteuert", "lauert", "Frauenkirche",
            "Gemäuer", "überquert", "lauern", "bedauere", "Umsatzsteuer",
            "untermauern", "Quedlinburg", "Guerilla", "Neuere", "Steuermann",
            "visuelle", "Frauenhilfe", "Regenschauer", "Plauen", "Frauenfußball",
            "Steuersatz", "steuerfrei", "Spandauer", "weiterzuentwickeln",
            "Hansruedi", "Genaue", "andauern", "Morgengrauen", "Quellenlage",
            "Neugebauer", "Einnahmequelle", "Enrique", "abenteuerliche",
            "zuerkannt", "baue", "Allgäuer", "Bäuerin", "Schroeder", "ueber",
            "intellektuell", "Bauernhaus", "Moers", "Betreuerin", "Frauenanteil",
            "feuerte", "Steuererhöhung", "Bäuerinnen", "bäuerliche",
            "Uefa-Cup", "Erbauer", "Spieldauer", "Schaeffler", "Uerdingen",
            "Amphoe", "Ungeheuer", "spirituelle", "Mineralölsteuer",
            "Poet", "Dauerbrenner", "Queensland", "Frauenquote", "Aargauer",
            "Immanuel", "Clique", "Bildhauerei", "Verteuerung", "Mehrwertsteuererhöhung",
            "Oerlikon", "Bequemlichkeit", "Goebel", "steuerten", "Rodríguez",
            "Steuersätze", "untermauert", "quelle", "Quecksilber", "True",
            "Frauentag", "Israelische", "Hoenig", "Goethestraße", "Scheuerfeld",
            "Soest", "Erbschaftsteuer", "Silhouette", "Monique", "Feuerstein"]
        .iter_mut()
        .map(|w| w.to_string())
        .collect();
        Self {
            link,
            entered_text: String::new(),
            edited_text: String::new(),
            ignore_words
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetText(s) => {
                self.entered_text = s;
                self.perform_substitution();
            }
        }
        true
    }

    fn view(&self) -> Html {
        let input_text = self.link.callback(|i: InputData| Msg::SetText(i.value));
        html! {
            <section class="app-container">
                <header>
                    <h1>{"Umlaute Substituter"}</h1>
                </header>
                <main>
                    <div>
                        <textarea type="text", placeholder="Enter Text",
                        oninput=input_text
                        rows="10", cols="80">
                            {&self.entered_text}
                        </textarea>
                    </div>
                </main>
                <output>
                    <textarea type="text"
                    rows="10", cols="80">
                        { &self.edited_text }
                    </textarea>
                </output>
            </section>
        }
    }

}

impl Model {
    fn perform_substitution(&mut self) {
        self.edited_text = replace_umlaute(
            &self.entered_text,
            &self.ignore_words
        );
    }
}