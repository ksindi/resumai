import React from 'react';
import UploadForm from './UploadForm';
import logo from './assets/resumai-logo.jpg';


function App() {
  return (
    <div style={styles.container}>
      <img src={logo} alt="logo" style={styles.logo} />

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
  logo: {
    maxWidth: '20%', // Ensures the logo isn't too large
    height: 'auto',
    marginBottom: '20px', // Adds some space between the logo and the heading
  }
};

export default App;
