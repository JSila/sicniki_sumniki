const btnFixWord =        window["fix-word"];
const btnFixText =        window["fix-text"];
const btnConfirmText =    window["confirm-text"];
const textarea =          window["text"];
const fixWordContainer =  window["fix-word-container"];
const correctWordSelect = window["correct-word"];
const newWordInput =      window["new-word"];
const btnConfirmNewWord = window["confirm-new-word"];

btnFixWord.addEventListener("click", onBtnFixWordClicked, false);
btnFixText.addEventListener("click", onBtnFixTextClicked, false);
btnConfirmText.addEventListener("click", onBtnConfirmTextClicked, false);
correctWordSelect.addEventListener('change', onCorrectWordSelected, false);
btnConfirmNewWord.addEventListener('click', onConfirmNewWordClicked, false);

var selectionStart = 0;
var selectionEnd = 0;


function onBtnFixWordClicked(event) {
    selectionStart = textarea.selectionStart;
    selectionEnd = textarea.selectionEnd;

    if (selectionEnd === selectionStart) {
        alert("Najprej izberi besedo!");
        return;
    }

    let word = textarea.value.substring(selectionStart, selectionEnd);

    if (word.split(/\s+/).length > 1) {
        alert("Izberi samo eno besedo!");
    }

    let lowercaseWord = word.toLowerCase();

    if (!lowercaseWord.includes("s") && !lowercaseWord.includes("c") && !lowercaseWord.includes("z") && !lowercaseWord.includes("š") && !lowercaseWord.includes("č") && !lowercaseWord.includes("ž")) {
        alert("Beseda ne vsebuje sicnikov ali sumnikov, tako da aplikacija z njo nima kaj delati!");
        return;
    }

    window.fetch(`http://localhost:3000/words/${word}`)
        .then(resp => resp.json())
        .then(data => {
            switch (data.similar_words.length) {
                case 0:
                    fixWordContainer.classList.remove("hidden");
                    newWordInput.classList.remove("hidden");
                    break;
                case 1:
                    textarea.value = `${textarea.value.substring(0, selectionStart)}${data.similar_words[0]}${textarea.value.substring(selectionEnd)}`;
                    break;
                default:
                    correctWordSelect.innerHTML = data.similar_words.map(w => `<option value="${w}">${w}</option>`);

                    fixWordContainer.classList.remove("hidden");
                    correctWordSelect.classList.remove("hidden");
                    newWordInput.classList.remove("hidden");
                    break;
            }
        });
}

function onCorrectWordSelected(event) {
    textarea.value = `${textarea.value.substring(0, selectionStart)}${event.target.selectedOptions[0].value}${textarea.value.substring(selectionEnd)}`;

    resetUI();
}

function onConfirmNewWordClicked(event) {
    const word = newWordInput.value;

    if (word.trim() === "") {
        alert("Pozabil si vpisati besedo!");
        return;
    }

    textarea.value = `${textarea.value.substring(0, selectionStart)}${word}${textarea.value.substring(selectionEnd)}`;

    resetUI();
}

function onBtnFixTextClicked(event) {
    const word = textarea.value;

    if (word.trim() === "") {
        alert("Pozabil si vpisati besedilo!");
        return;
    }

    fetch("http://localhost:3000/text/fix", {
        method: "POST",
        body: JSON.stringify({text: textarea.value})
    })
    .then(resp => resp.json())
    .then(resp => {
        textarea.value = resp.text;
    })
    .finally(() => {
        resetUI();
    });
}

function onBtnConfirmTextClicked(event) {
    const word = textarea.value;

    if (word.trim() === "") {
        alert("Pozabil si vpisati besedilo!");
        return;
    }

    fetch("http://localhost:3000/text/confirm", {
        method: "POST",
        body: JSON.stringify({text: textarea.value})
    })
    .then(() => {
        alert("Hvala za prijaznost! :)")
    })
    .finally(() => {
        resetUI();
    });
}

function resetUI() {
    selectionStart = 0;
    selectionEnd = 0;
    fixWordContainer.classList.add("hidden");
    correctWordSelect.classList.add("hidden");
    newWordInput.classList.add("hidden");
}