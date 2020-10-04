import React, {useRef} from 'react'
import ReactDOM from 'react-dom'
import {Provider, useDispatch, useSelector} from 'react-redux'

import store, {setText} from './store'

import Alert from "./components/Alert";
import Actions, {ACTION} from "./components/Actions";
import FixWord from "./components/FixWord";
import Instructions from "./components/Instructions";

const App = function() {

    const action = useSelector(store => store.action)
    const alert = useSelector(store => store.alert)
    const text = useSelector(store => store.text)
    const dispatch = useDispatch()

    const textarea = useRef()

    return (
        <>
            {alert && <Alert />}
            <textarea ref={textarea} value={text} onChange={e => dispatch(setText(e.target.value))} />
            <Actions textarea={textarea} />
            {action === ACTION.FIX_WORD && <FixWord />}
            {action === ACTION.INSTRUCTIONS && <Instructions />}
        </>
    )
}

const Root = (
    <Provider store={store}>
        <App/>
    </Provider>
)

ReactDOM.render(Root, document.querySelector(".container"))