import React from "react";

import {useDispatch, useSelector} from "react-redux";
import {setAction, setAlert, setSimilarWords, setText} from "../store";

export const ACTION = {
    FIX_WORD: "fix_word",
    INSTRUCTIONS: "instructions",
    AUTOFIX: "autofix",
    CONFIRM_TEXT: "confirm_text",
}

const Actions = props => {

    const action = useSelector(store => store.action)
    const text = useSelector(store => store.text)
    const dispatch = useDispatch()

    const fixWord = () => {
        const {selection, error} = getSelection(props.textarea.current)
        if (error) {
            dispatch(setAlert({ message: error, type: "warning"}))
            return
        }

        window.fetch(`${process.env.BACKEND_APP_URL}/words/${selection.word}`)
            .then(resp => resp.json())
            .then(data => {
                switch (data.similar_words.length) {
                    case 0:
                        dispatch(setAction({action: ACTION.FIX_WORD, selection}))
                        break;
                    case 1:
                        dispatch(setText(`${text.substring(0, selection.selectionStart)}${data.similar_words[0]}${text.substring(selection.selectionEnd)}`))
                        break;
                    default:
                        dispatch(setSimilarWords(data.similar_words))
                        dispatch(setAction({action: ACTION.FIX_WORD, selection}))
                        break;
                }
            });
    }

    const toggleInstructions = () => {
        dispatch(setAction(action === ACTION.INSTRUCTIONS ? undefined : ACTION.INSTRUCTIONS))
    }

    const confirmText = () => {
        if (!text.trim()) {
            dispatch(setAction())
            dispatch(setAlert({ message: "Pozabili ste vpisati besedilo!", type: "warning"}))
            return
        }

        window.fetch(`${process.env.BACKEND_APP_URL}/text/confirm`, {
            method: "POST",
            body: JSON.stringify({text: text})
        }).then(() => {
            dispatch(setAlert({ message: "Operacija je bila upešno izvedena.", type: "success"}))
        }).finally(() => {
            dispatch(setAction())
        })
    }

    const autofixText = () => {
        if (!text.trim()) {
            dispatch(setAction())
            dispatch(setAlert({ message: "Pozabili ste vpisati besedilo!", type: "warning"}))
            return
        }
        window.fetch(`${process.env.BACKEND_APP_URL}/text/fix`, {
            method: "POST",
            body: JSON.stringify({text: text})
        })
        .then(resp => resp.json())
        .then(resp => {
            dispatch(setText(resp.text))
        })
    }

    return (
        <div className="actions">
            <button
                data-action={ACTION.FIX_WORD}
                onClick={fixWord}>
                Popravi besedo
            </button>
            <button
                data-action={ACTION.AUTOFIX}
                onClick={autofixText}> Samodejno popravi besedilo</button>
            <button
                data-action={ACTION.CONFIRM_TEXT}
                onClick={confirmText}
                title="V primeru ročno popravljenega besedila">
                Potrdi pravilnost
            </button>
            <button
                data-action={ACTION.INSTRUCTIONS}
                onClick={toggleInstructions}
                title="Navodila za uporabo aplikacije">
                &#8505;
            </button>
        </div>
    )
}

function getSelection(el) {
    const selectionStart = el.selectionStart;
    const selectionEnd = el.selectionEnd;

    if (selectionEnd === selectionStart) {
        return { error: "Najprej izberite besedo!"}
    }

    let word = el.value.substring(selectionStart, selectionEnd);

    if (word.split(/\s+/).length > 1) {
        return { error: "Izberite samo eno besedo!"}
    }

    let lowercaseWord = word.toLowerCase();

    if (!lowercaseWord.includes("s") && !lowercaseWord.includes("c") && !lowercaseWord.includes("z") && !lowercaseWord.includes("š") && !lowercaseWord.includes("č") && !lowercaseWord.includes("ž")) {
        return { error: "Beseda ne vsebuje sičnikov ali šumnikov, tako da aplikacija z njo nima kaj delati!"}
    }

    return { selection: {word, selectionStart, selectionEnd} }
}

export default Actions