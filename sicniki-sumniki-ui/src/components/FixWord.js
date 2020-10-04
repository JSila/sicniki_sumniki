import React from "react";
import {useDispatch, useSelector} from "react-redux";
import {setAction, setAlert, setCorrectWord, setNewWord} from "../store";

const FixWord = () => {
    const similarWords = useSelector(store => store.similarWords)
    const newWord = useSelector(store => store.newWord)
    const dispatch = useDispatch()

    const handleSelect = e => {
        dispatch(setCorrectWord({ word: e.target.value, reset: true }))
    }

    const handleClick = e => {
        const newWord = e.target.value
        if (!newWord.trim()) {
            dispatch(setAlert({ message: "Pozabil si vpisati besedo!", type: "warning"}))
            return
        }

        dispatch(setCorrectWord({ word: newWord, reset: true }))
    }

    return (
        <div className="fix-word">
            <select size="5" onChange={handleSelect}>
                {similarWords.map(w => <option key={w} value={w}>{w}</option>)}
            </select>
            <input type="text" value={newWord} onChange={e => dispatch(setNewWord(e.target.value))} placeholder="Ne obstaja? Vpisi jo tu..."/>
            <button className="fix-word_btn" onClick={handleClick}>Potrdi</button>
        </div>
    )
}

export default FixWord