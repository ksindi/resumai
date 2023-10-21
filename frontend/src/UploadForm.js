import React, { useCallback, useState, useEffect } from 'react';
import ReactMarkdown from 'react-markdown';

import {
    getUploadURL,
    uploadFileToURL,
    fetchEvaluation,
    saveResultToCookie,
    loadResultsFromCookie
} from './utils';
import DropzoneArea from './Dropzone';
import PreviousResults from './PreviousResults';


const UploadForm = () => {
    const [file, setFile] = useState(null);
    const [evaluation, setEvaluation] = useState(null);
    const [isLoading, setIsLoading] = useState(false);
    const [previousResults, setPreviousResults] = useState([]);

    useEffect(() => {
        loadPreviousResultsFromCookie();
    }, []);

    const loadPreviousResultsFromCookie = async () => {
        const savedResults = loadResultsFromCookie();
        if (savedResults.length > 0) {
            console.log("Found previous results, loading from cookie");
            console.log("Previous results", savedResults);
            const updatedResults = await Promise.all(savedResults.map(async (result) => {
                const evaluationText = await fetchEvaluation(result.evaluationId);
                // HACK: Remove "Assistant: " prefix from evaluation text
                return { ...result, evaluationText: evaluationText.replace("Assistant: ", "") };
            }));
            setPreviousResults(updatedResults);
        } else {
            console.log("No previous results found, fetching example evaluation");
            const exampleEvaluationId = "3beb8090-d11a-4852-8a5c-a9ae6cc4d04e";
            const evaluationText = await fetchEvaluation(exampleEvaluationId);
            console.log("Example evaluation response", evaluationText);
            const created = new Date().toLocaleString();
            const exampleEvaluation = {
                fileName: "example-resume.pdf",
                evaluationId: exampleEvaluationId,
                evaluationText: evaluationText.replace("Assistant: ", ""),
                created,
            };

            setPreviousResults([exampleEvaluation]);
        }
    };

    useEffect(() => {
        const warnBeforeUnload = (e) => {
            if (isLoading) {
                e.preventDefault();
                e.returnValue = 'Your evaluation is still in progress. Do you want to leave?';
            }
        };

        window.addEventListener('beforeunload', warnBeforeUnload);
        return () => window.removeEventListener('beforeunload', warnBeforeUnload);
    }, [isLoading]);

    const onDrop = useCallback(acceptedFiles => {
        console.log("Accepted files", acceptedFiles);
        const file = acceptedFiles[0];
        setFile(file);
    }, []);

    const handleFileUpload = async (e) => {
        e.stopPropagation(); // Stop the event from bubbling up

        try {
            setIsLoading(true);

            const getUploadURLResponse = await getUploadURL();
            console.log("Received upload URL", getUploadURLResponse);
            const { upload_url, evaluation_id } = getUploadURLResponse.data;
            await uploadFileToURL(upload_url, file);

            const created = new Date().toLocaleString();
            const previousResultsFromCookie = loadResultsFromCookie();
            saveResultToCookie(
                previousResultsFromCookie,
                { fileName: file.name, evaluationId: evaluation_id, created }
            );

            const evaluationText = await fetchEvaluation(evaluation_id);
            console.log("Received evaluation response", evaluationText);
            setEvaluation(evaluationText.replace("Assistant: ", ""));

            setIsLoading(false);
        } catch (error) {
            setIsLoading(false);
            alert(error.message);
        }
    };

    if (isLoading) {
        return <div style={styles.centeredText}>Processing... (can take up to 2 minutes)</div>;
    }

    if (evaluation) {
        return (
            <div style={styles.responseContainer}>
                <ReactMarkdown>{evaluation}</ReactMarkdown>
            </div>
        );
    }

    return (
        <div>
            <DropzoneArea file={file} onDrop={onDrop} handleFileUpload={handleFileUpload} />
            <PreviousResults results={previousResults} />
        </div >
    );

};

const styles = {
    centeredText: {
        height: '100vh',
        textAlign: 'center',
    },
    responseContainer: {
        padding: '20px',
        margin: '0 auto',
        lineHeight: '1.5',
        maxWidth: '800px',
        textAlign: 'left',
    },
};

export default UploadForm;
