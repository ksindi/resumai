import axios from 'axios';
import Cookies from 'js-cookie'; // Assuming you are using 'js-cookie'

const POLL_INTERVAL = 5000; // 5 seconds
const MAX_POLL_DURATION = 120000; // 2 minutes
const COOKIE_NAME = 'evaluationsCookie';

const apiClient = axios.create({
    baseURL: 'https://3yyyw6og8d.execute-api.us-east-1.amazonaws.com/prod'
});

export const getUploadURL = async () => {
    return await apiClient.post('/upload');
};

export const uploadFileToURL = async (uploadUrl, file) => {
    return await axios.put(uploadUrl, file, { headers: { 'Content-Type': 'application/pdf' } });
};

export const fetchEvaluation = async (evaluationId, elapsed = 0) => {
    try {
        const response = await apiClient.get(`/evaluations/${evaluationId}`);

        return response.data;
    } catch (error) {
        // If we encounter a 404 and haven't polled for 2 minutes, retry after POLL_INTERVAL
        if (error.response && error.response.status === 404 && elapsed < MAX_POLL_DURATION) {
            console.log('Retrying fetchEvaluation in', POLL_INTERVAL, 'ms...');
            // Wait for the POLL_INTERVAL duration before retrying
            await new Promise(resolve => setTimeout(resolve, POLL_INTERVAL));
            return await fetchEvaluation(evaluationId, elapsed + POLL_INTERVAL);
        }

        // If it's another error or if we've polled for 2 minutes, throw the error
        throw error;
    }
};

export const saveResultToCookie = (previousResults, newResult) => {
    const updatedResults = [...previousResults, newResult];
    Cookies.set(COOKIE_NAME, JSON.stringify(updatedResults), { expires: 365 }); // Save for 1 year
    return updatedResults;
};

export const loadResultsFromCookie = () => {
    const savedResults = Cookies.get(COOKIE_NAME);
    return savedResults ? JSON.parse(savedResults) : [];
};
