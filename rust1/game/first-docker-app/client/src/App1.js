import React, { useCallback, useState, useEffect } from "react";
import axios from "axios";
import './App.css';

// import "./MainComponent.css";

const App1 = () => {
  const [values, setValues] = useState([]);
  const [value, setValue] = useState("");

  const getAllNumbers = useCallback(async () => {
    // we will use nginx to redirect it to the proper URL
    const data = await axios.get("/api_postgres/values/all");
    console.log("data")
    console.log( data)
    setValues(data.data.rows.map(row =>  ({id:row.id, task:row.task})));
  }, []);

  const saveNumber = useCallback(
    async event => {
      event.preventDefault();

      await axios.post("/api_postgres/values", {
        value
      });
      document.location.reload();

      // setValue("");
      // getAllNumbers();
    },
    [value, getAllNumbers]
  );

  const deleteNumber = useCallback(
    async id =>
    {
      console.log(id)

      if (window.confirm("Do you want to delete this task?"))
      {
        await axios.delete(`/api_postgres/delete/${id}`)
          .then(() => document.location.reload())
          .catch((error) => console.error(error));  

      }
      document.location.reload();
      

    }
  )






  useEffect(() => {
    getAllNumbers();
    console.log("USE EFFECT")
    console.log(values)
  }, []);

  const card = values.map((val, key) => (
    <React.Fragment key={key}>
      <div className="m-2 bg-blue-200 rounded-t-3xl rounded-b-sm shadow-md w-5/12 ">
        <div className='p-6 overflow-hidden flex justify-between items-center'>
          <h2 className='text-base text-gray-800 m-2 whitespace-normal'>{val.task}</h2>
          <button className="bg-red-400 text-sm text-gray-800 px-4 py-2 rounded-full" onClick={()=>deleteNumber(val.id)}>Delete</button>
        </div>
      </div>
    </React.Fragment>
  ));


  return (
    <div className="text-center">

      <h1 className='text-3xl font-bold text-gray-800 text-center mt-10 mb-6 tracking-widest leading-snug'>Task List using postgres</h1>
      <form className="form" onSubmit={saveNumber}>
        {/* <label>Enter your value: </label> */}
        <input
          value={value}
          placeholder="Enter task"
          onChange={event => {
            setValue(event.target.value);
          }} className=' mx-2 tracking-widest leading-snug bg-gray-100 border border-gray-300 text-gray-900 text-sm rounded-full focus:outline-blue-500 focus:border-blue-500 p-3.5'
        />
        <button className="mx-2 tracking-widest leading-snug bg-blue-200 p-3.5 rounded-full focus:bg-blue-300 focus:outline-blue-500">Submit</button>
      </form>

      {/* <Container>
        <Row>{card}</Row>
      </Container> */}

      <div className="flex flex-col overflow-y-auto justify-center items-center">
        {card}
      </div>



    </div>
  );
};

export default App1;

