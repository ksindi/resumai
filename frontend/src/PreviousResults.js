import ReactMarkdown from 'react-markdown';

import {
    handleDownload,
    handleDelete,
} from './utils';
import './PreviousResults.css';

const PreviousResults = ({ results }) => (
    <div style={styles.resultsContainer}>
        {results.length > 0 && <h2>Previous Results</h2>}
        {results.sort((a, b) => new Date(b.created) - new Date(a.created)).map((result) => (
            <div className="rowContainer" key={result.evaluationId}>
                <details>
                    <summary>
                        {result.fileName} - {result.created}
                        <span onClick={() => handleDownload(result.fileName, result.evaluationId)}>📥</span>
                        <span onClick={() => handleDelete(result.evaluationId)}>🗑️</span>
                    </summary>
                    <div style={styles.responseContainer}>
                        <ReactMarkdown>{result.evaluationText}</ReactMarkdown>
                    </div>
                </details>
            </div>
        ))}
    </div>
);

const styles = {
    responseContainer: {
        padding: '20px',
        margin: '0 auto',
        lineHeight: '1.5',
        maxWidth: '800px',
    },
};

export default PreviousResults;