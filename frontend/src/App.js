import React from 'react';
import UploadForm from './UploadForm';
import logo from './assets/resumai-logo.jpg';
import './App.css';


function App() {
  return (
    <div style={styles.container}>
      <img src={logo} alt="logo" className="logo-image" />

      <h1>Get Expert Feedback on Your Resume! (beta)</h1>
      <p style={styles.action}>Upload your resume and receive professional feedback within minutes.</p>

      <div className="main-container">
        <UploadForm />

        <FAQSection />
      </div>
    </div>
  );
}

const FAQSection = () => {
  return (
    <div style={styles.faqContainer}>
      <h2>FAQ</h2>
      <ul style={styles.faqList}>
        <li>
          <strong>What is ResumAI?</strong>
          <p>ResumAI is a tool that provides feedback on your resume. It's a hobby project that you shouldn't take too seriously. My goal is help people craft better resumes.</p>
        </li>
        <li>
          <strong>How do I use it?</strong>
          <p>Upload your resume and wait for feedback.</p>
        </li>
        <li>
          <strong>How long does it take?</strong>
          <p>Feedback is usually provided within minutes of uploading.</p>
        </li>
        <li>
          <strong>What kind of feedback do I get?</strong>
          <p>ResumAI provides feedback on your resume's content and format. See the example.</p>
        </li>
        <li>
          <strong>What format does my resume need to be in?</strong>
          <p>ResumAI currently only accepts PDF.</p>
        </li>
        <li>
          <strong>How many resumes can I upload?</strong>
          <p>You can upload as many resumes as you like!</p>
        </li>
        <li>
          <strong>Who is this for?</strong>
          <p>The prompt is currently tuned to evaluate software engineers.</p>
        </li>
        <li>
          <strong>How much does it cost?</strong>
          <p>ResumAI is free to use.</p>
        </li>
        <li>
          <strong>How does this work?</strong>
          <p>ResumAI uses OpenAI's GPT3 API to provide feedback on your resume.</p>
        </li>
        <li>
          <strong>How long is my data retained?</strong>
          <p>Your resume is only retained for 1 day after which it's deleted.</p>
        </li>
        <li>
          <strong>Is this the final version?</strong>
          <p>No, this is a beta version as I'm still tweaking the right prompt.</p>
        </li>
        <li>
          <strong>Is my resume shared with anyone?</strong>
          <p>No, your resume is not shared with anyone outside of OpenAI's GPT API.</p>
        </li>
        <li>
          <strong>I do I provide feedback about the app?</strong>
          <p>Please send your feedback to kysindi [at] gmail dot com.</p>
        </li>
        <li>
          <strong>Where does the code live?</strong>
          <p>You can checkout out here https://github.com/ksindi/resumai.</p>
        </li>
      </ul>
    </div>
  );
};


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
  faqContainer: {
    width: '80%',
    marginTop: '50px',
    padding: '20px 0',
  },
  faqList: {
    listStyleType: 'none',
    padding: 0,
  },
};

export default App;
