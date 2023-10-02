import React from 'react';
import UploadForm from './UploadForm';

function App() {
  return (
    <div style={styles.container}>
      <h1>PDF Upload</h1>
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
