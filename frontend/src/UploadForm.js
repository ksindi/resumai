import React, { useCallback, useState } from 'react';
import { useDropzone } from 'react-dropzone';
import axios from 'axios';

const UploadForm = () => {
    const [file, setFile] = useState(null);

    const onDrop = useCallback(acceptedFiles => {
        const file = acceptedFiles[0];
        setFile(file);
    }, []);

    const { getRootProps, getInputProps } = useDropzone({
        onDrop,
        accept: 'application/pdf', // Accept only PDF files
    });

    const uploadFile = async () => {
        const formData = new FormData();
        formData.append('file', file);

        try {
            const response = await axios.post('/api/upload', formData);
            alert('File uploaded successfully to: ' + response.data.fileUrl);
        } catch (error) {
            alert('Error uploading file.');
        }
    };

    return (
        <div {...getRootProps()} style={styles.dropzone}>
            <input {...getInputProps()} />
            <p>Drag & drop a PDF here, or click to select one</p>
            {file && <p>Selected file: {file.name}</p>}
            <button onClick={uploadFile} disabled={!file} style={styles.button}>
                Upload
            </button>
        </div>
    );
};

const styles = {
    dropzone: {
        border: '2px dashed #cccccc',
        borderRadius: '4px',
        padding: '20px',
        textAlign: 'center',
        cursor: 'pointer',
    },
    button: {
        marginTop: '10px',
    },
};

export default UploadForm;
