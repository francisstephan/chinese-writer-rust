pub fn ziform() -> String {
    let form = r##"
	  <form hx-post="/zilist" hx-target="#content" hx-swap="innerHTML" >
		    <label for="carac">Character:</label>
			<enctype="application/x-www-form-urlencoded">
		    <input id="carac" name="carac" type="text" autofocus required minlength="1" maxlength="1">
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

pub fn pyform() -> String {
    let form = r##"
        <form hx-post="/pylist" hx-target="#content" hx-swap="innerHTML" >
		    <label for="pinyin">Pinyin+tone (using pattern ^[a-z,ü]+[0-4]?) :</label>
		    <input id="pinyin" name="pinyin_ton" type="text" pattern="^[a-z,ü]+[0-4]?" autofocus>
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

pub fn whs() -> String {
    let form = r##"
        <h2>
	    Use latin keyboard, write text in chinese characters (hanzi, 汉字)
        </h2>
        <p>1. Enter pinyin with tone in the "Enter" textarea below. A list of possible hanzi appears.<br />
           2. Select hanzi from list by clicking on it.<br />
           The selected zi gets added to the <b>Result hanzi text</b>.<br />
           To add more hanzi to the text, repeat steps 1 & 2 again</p>

        <form id="postzi" hx-post="/candidatelist" hx-target="#zilist" hx-swap="innerHTML">
            <label for="pinyin">Enter pinyin+tone (press / or space after pinyin if tone unknown) :</label>
            <input type='text' id='pinyin' name='pinyin_ton' size='10' autofocus oninput='convertToZi()'>
            <button id="subpy" style="display:none" type="submit"></button>
        </form>
        <p id="resultat"><b>Result hanzi text :</b>
            <input type='text' id='zistring' size='40'></p>

        <button class = "Addzi" onclick='copyTextToClipboard()' >Copy to clipboard</button>
        <button class = "Addzi" onclick='reset()' >Reset</button>
        <button class = "Addzi" onclick="lookup(document.getElementById('zistring').value)" >Google Translate text</button>
        <button class = "Addzi" onclick="lookupWrittenChinese(document.getElementById('zistring').value)" >Lookup Written字Chinese</button>
        <div id="zilist"></div>
    "##;
    String::from(form)
}

pub fn zistringform() -> String {
    let form = r##"
	    <p id="formhead">Enter hanzi string to parse :</p>
        <form hx-post="/stringparse" hx-target="#content" hx-swap="innerHTML" >
			<enctype="application/x-www-form-urlencoded">
		    <input id="zistr" name="zistr" type="text" autofocus required size="80" minlength="1" maxlength="400">
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	    </form>
	"##;
    String::from(form)
}
