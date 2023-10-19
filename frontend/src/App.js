import React from 'react';
import UploadForm from './UploadForm';
import logo from './assets/resumai-logo.jpg';
import './App.css';


function App() {
  return (
    <div style={styles.container}>
      <img src={logo} alt="logo" className="logo-image" />

      <h1>Get Expert Feedback on Your Resume!</h1>
      <p style={styles.action}>Upload your resume and receive professional feedback within minutes.</p>

      <UploadForm />
    </div>
  );
}

const styles = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    paddingTop: '50px',
  },
  action: {
    textAlign: 'center',
    maxWidth: '600px',
    padding: '0 20px',
  },
};

export default App;
