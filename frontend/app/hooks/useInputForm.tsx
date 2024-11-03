import {useState, ChangeEvent} from "react";

function useInputForm() {
    const [value, setValue] = useState("");
    const onChangeInputForm = (event: ChangeEvent<HTMLInputElement>) => {
        setValue(event.currentTarget.value);
    }
    return [value, onChangeInputForm];
}

export default useInputForm;