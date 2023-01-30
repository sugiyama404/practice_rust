import { useEffect, useState } from 'react'
import './App.css'
import axios from 'axios';

function App() {
  const url = "http://localhost/api/todo";
  const [inputValue, setInputValue] = useState("");
  const [todos, setTodos] = useState<Todo[]>([]);

  type Todo = {
    id: number,
    content: string,
  }

  const onWriting = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setInputValue(e.target.value);
  };

  const onSave = (e: { preventDefault: () => void }) => {
    e.preventDefault();
    if (!inputValue) { return; }

    axios.post('/api/todo', { content: inputValue })
      .then(function (response) {
        console.log(response.data);
        axios.get(url).then((res) => {
          setTodos([...res.data]);
        }).catch(error => {
          console.log(error);
        });
      })

    setInputValue("");
  };

  const onDelete = (uid: number) => {

    axios.delete('/api/todo/' + String(uid))
      .then(function (response) {
        console.log(response.data);
        axios.get(url).then((res) => {
          setTodos([...res.data]);
        }).catch(error => {
          console.log(error);
        });
      })
  };

  useEffect(() => {
    axios.get(url).then((res) => {
      setTodos([...res.data]);
    }).catch(error => {
      console.log(error);
    });
  }, [])
  return (
    <div className="App">
      <h1>リマインダー</h1>
      <form onSubmit={(e) => onSave(e)}>
        <input type="text" style={{ width: "400px", height: "30px" }} value={inputValue} onChange={(e) => onWriting(e)} />
        <input type="submit" style={{ width: "100px", height: "35px" }} value="送信" />
      </form>
      <br></br>
      <table border={1}>
        {todos.map((arr: Todo) => (
          <tr>
            <td style={{ textAlign: 'left' }} width={500}>{arr.content}</td>
            <td>
              <button onClick={() => onDelete(arr.id)}>削除</button>
            </td>
          </tr>
        ))}
      </table>
    </div >
  )
}

export default App
