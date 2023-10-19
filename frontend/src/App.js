import React from 'react';
import UploadForm from './UploadForm';
import logo from './assets/resumai-logo.jpg';


function App() {
  return (
    <div style={styles.container}>
      <img src={logo} alt="logo" style={styles.logo} />

      <h1>Get Expert Feedback on Your Resume!</h1>
      <p>Upload your resume and receive professional feedback within minutes.</p>

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
};

export default App;
