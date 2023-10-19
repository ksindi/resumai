import { useDropzone } from 'react-dropzone';

const DropzoneArea = ({ file, onDrop, handleFileUpload }) => {
    const { getRootProps, getInputProps } = useDropzone({
        onDrop,
        accept: 'application/pdf',
    });

    return (
        <div {...getRootProps()} style={styles.dropzone}>
            <input {...getInputProps()} />
            <p>Drag & drop a resume PDF here, or click to select one</p>
            <p style={styles.instructionText}>Accepted format: PDF. Maximum size: 5MB.</p>
            {file && <p>Selected file: {file.name}</p>}
            <button
                onClick={(e) => handleFileUpload(e)}
                disabled={!file}
                style={styles.button}>
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
}

export default DropzoneArea;