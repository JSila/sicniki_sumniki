import React from "react";
import {useDispatch, useSelector} from "react-redux";
import {setAlert} from "../store";

const Alert = () => {
    const alert = useSelector(store => store.alert)
    const dispatch = useDispatch()

    React.useEffect(() => {
        setTimeout(() => {
            dispatch(setAlert())
        }, 5000)
    })

    return (
        <div className={`alert alert-${alert.type}`}>{alert.message}</div>
    )
}

export default Alert