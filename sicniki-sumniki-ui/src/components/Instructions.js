import React from "react";

const Instructions = () => (
    <div className="instructions alert alert-info" id="instructions">
        <div className="instructions_title">Navodila za uporabo aplikacije</div>
        <div id="instructions_content" className="instructions_content hidden">
            <p>
                Aplikacija omogoča, da se vnesenemu besedilu popravi besede, ki so namesto s šumniki napisane s sičniki. Če npr. uporabljate angleško tipkovnico, bi pa vseeno rad, da je slovnično pravilno, vendar je preklapljanje med tipkovnicami v različnih jezikih zamudno..                
            </p>
            <p>
                S prvo akcijo <em>Popravi besedo</em> lahko popravljate besedo eno po eno. Označite besedo, kliknite na omenjeno akcijo in beseda se bo ob enem kandidatu popravila, ob večih kandidatih pa se bodo le ti izpisali. Kliknite na ustreznega, da se beseda popravi ali pa vpiši novo ter vnos potrdite s pritiskom na gumb Potrdi.    
            </p>
            <p>
                Z drugo akcijo <em>Samodejno popravi besedilo</em> bo sistem sam poskušal popraviti besedilo. Pri tem bo zamenjal besede s sičniki s šumniki le v primeru, da za besedo obstaja le en kandidat.
            </p>
            <p>
                Z zadnjo akcijo <em>Potrdi pravilnost</em> se bodo posodobljene besede shranile v bazo aplikacije. Uporabnost te akcije je v tem, da pri naslednji uporabi aplikacije sistem zna biti bolj v pomoč, saj bo kot predlog pri prej omenjenima akcijama uporabile posodobljene besede.            
            </p>
        </div>
    </div>
)

export default Instructions