/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
// import React, { Component } from 'react';
import './App.css';
import axios from 'axios';
// import { Button, Container, Card, Row } from 'react-bootstrap'


import React, { useState, useEffect } from 'react';
// import axios from 'axios';

const App = () => {
  const [bookName, setBookName] = useState('');
  const [review, setReview] = useState('');
  const [fetchData, setFetchData] = useState([ ]);
  const [reviewUpdate, setReviewUpdate] = useState('');

  const handleChange = (event) => {
    const { name, value } = event.target;
    if (name === 'setBookName') {
      setBookName(value);
    } else if (name === 'setReview') {
      setReview(value);
    }
  };

  const handleChange2 = (event) => {
    setReviewUpdate(event.target.value);
  };

  const submit = () => {
    axios.post('/api_mysql/insert', { setBookName: bookName, setReview: review })
      .then(() => { alert('success post'); })
      .catch((error) => { console.error(error); });
    console.log({ bookName, review });
    document.location.reload();
  };

  const deleteItem = (id) => {
    if (confirm('Do you want to delete? ')) {
      axios.delete(`/api_mysql/delete/${id}`)
        .then(() => document.location.reload())
        .catch((error) => console.error(error));
    }
    document.location.reload();
  };

  const edit = (id) => {
    axios.put(`/api_mysql/update/${id}`, { reviewUpdate: reviewUpdate })
      .then(() => document.location.reload())
      .catch((error) => console.error(error));
      document.location.reload();
  };

  useEffect(() => {
    axios.get('/api_mysql/get')
      .then((response) => {
        setFetchData(response.data);
      })
      .catch((error) => {
        console.error(error);
      });
  }, []);

  const card = fetchData.map((val, key) => (
    <React.Fragment key={key}>
      <div className="m-2 bg-purple-200 rounded-t-3xl rounded-b-2xl shadow-md w-72 h-fit">
        <div className='p-6'>
          <h2 className='text-2xl font-bold text-gray-800 mb-2'>{val.book_name}</h2>
          <p className='text-gray-700 mb-6'>{val.book_review}</p>
          <input className='w-full border border-gray-300 mb-4 tracking-widest leading-snug bg-gray-100 text-gray-900 text-sm rounded-full focus:ring-blue-500 focus:border-blue-500 p-3.5' name="reviewUpdate" onChange={handleChange2} placeholder="Update Review" />
          <button className=" bg-green-300 text-gray-800 px-4 py-2 rounded-full mr-4" onClick={() => edit(val.id)}>Update</button>
          <button className="bg-red-400 text-gray-800 px-4 py-2 rounded-full" onClick={() => deleteItem(val.id)}>Delete</button>
        </div>
      </div>
    </React.Fragment>
  ));

  return (
    <div className=" text-center h-5/12">
      <h1 className='text-3xl font-bold text-gray-800 text-center mt-10 mb-6 tracking-widest leading-snug'>Book Review using MySQL</h1>
      <div className="form">
        <input className='mx-2 tracking-widest leading-snug bg-gray-100 border border-gray-300 text-gray-900 text-sm rounded-full focus:outline-blue-500 focus:border-blue-500 p-3.5' name="setBookName" placeholder="Enter Book Name" onChange={handleChange} />
        <input className='mx-2 tracking-widest leading-snug bg-gray-100 border border-gray-300 text-gray-900 text-sm rounded-full focus:outline-blue-500 focus:border-blue-500 p-3.5' name="setReview" placeholder="Enter Review" onChange={handleChange} />
        <button className="mx-2 tracking-widest leading-snug bg-purple-200 p-3.5 rounded-full focus:bg-purple-300 focus:outline-purple-500" variant="primary" onClick={submit}>Submit</button>

      </div>

      <br /><br />

      <div className='flex flex-row overflow-x-auto'>
        <div className="flex flex-shrink-0 rounded-lg bg-white p-4 shadow-lg mr-4 h-fit">
          {card}
          </div>
      </div>
    </div>
  );
};

export default App;