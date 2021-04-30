import {createSlice, configureStore} from '@reduxjs/toolkit'

const appSlice = createSlice({
    name: "app",
    initialState: {
        action: "",
        alert: null,
        text: "",
        selection: "",
        similarWords: [],
        newWord: ""
    },
    reducers: {
        setAction(state, action) {
            if (typeof action.payload === "object") {
                state.action = action.payload.action
                if ("selection" in action.payload) {
                    state.selection = action.payload.selection
                }
            } else {
                state.action = action.payload
            }
        },
        setAlert(state, action) {
            state.alert = action.payload
        },
        setText(state, action) {
            state.text = action.payload
        },
        setSelection(state, action) {
            state.selection = action.payload
        },
        setSimilarWords(state, action) {
            state.similarWords = action.payload
        },
        setCorrectWord(state, action) {
            const word = action.payload.word;
            state.text = `${state.text.substring(0, state.selection.selectionStart)}${word}${state.text.substring(state.selection.selectionEnd)}`

            if (action.payload.reset) {
                state.selection = ""
                state.action = ""
                state.similarWords = []
                state.newWord = ""
            }
        },
        setNewWord(state, action) {
            state.newWord = action.payload
        }
    }
})

export const { setAlert, setText, setAction, setSelection, setSimilarWords, setCorrectWord, setNewWord } = appSlice.actions

export default configureStore({
    reducer: appSlice.reducer
})