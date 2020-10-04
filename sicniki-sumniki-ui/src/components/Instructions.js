import React from "react";

const Instructions = () => (
    <div className="instructions alert alert-info" id="instructions">
        <div className="instructions_title">Navodila za uporabo aplikacije</div>
        <div id="instructions_content" className="instructions_content hidden">
            <p>Aplikacija omogoca, da se vnesenemu besedilu popravi besede, ki so namesto s sumniki napisane s sicniki.
                Ce npr. uporabljas anglesko tipkovnico, a bi vseeno rad, da je slovnicno pravilno.</p>
            <p>S prvo akcijo <em>Popravi besedilo</em> lahko popravljas besedo eno po eno. Oznaci besedo, klikni na
                omenjeno akcijo in beseda se bo ob enem kandidatu popravila, ob vecih kandidatih pa se bodo le ti
                izpisali. Klikni na ustreznega, da se beseda popravi ali pa vpisi novo ter vnos potrdi s pritiskom na
                gumb <em>Potrdi</em>.</p>
            <p>Z drugo akcijo <em>Samodejno popravi besedilo</em> bo sistem sam poskusal popraviti besedilo. Pri tem bo
                zamnejal besede s sicniki s sumniki le v primeru, da za besedo obstaja le en kandidat.</p>
            <p>Z zadnjo akcijo, <em>Potrdi pravilnost</em>, se bodo posodobljene besede shranile v bazo aplikacije.
                Uporabnost te akcije je v tem, da pri naslednji uporabi aplikacije sistem zna biti bolj v pomoc, saj bo
                kot predlog pri prej omenjenima akcijama uporabile posodobljene besede.</p>
        </div>
    </div>
)

export default Instructions