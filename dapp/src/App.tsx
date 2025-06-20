import React, { FC, useState, useEffect, useCallback } from 'react';
import { BrowserProvider, Contract } from 'ethers';
import SensorDataRecorderAbi from './contracts/SensorDataRecorder.json';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import Papa from 'papaparse';
import { saveAs } from 'file-saver';

const CONTRACT_ADDRESS = process.env.REACT_APP_CONTRACT_ADDRESS || 'YOUR_DEPLOYED_CONTRACT_ADDRESS';

interface SensorReading {
  timestampUnixNano: number;
  sensorId: string;
  location: string;
  processStage: string;
  temperatureCelsiusScaled: number;
  humidityPercentScaled: number;
  recorder: string;
}

const App: FC = () => {
  const [provider, setProvider] = useState<BrowserProvider | null>(null);
  const [contract, setContract] = useState<Contract | null>(null);
  const [account, setAccount] = useState<string | null>(null);
  const [sensorReadings, setSensorReadings] = useState<SensorReading[]>([]);
  const [loading, setLoading] = useState(false);
  const [, setError] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [showHistory, setShowHistory] = useState(false);

  useEffect(() => {
    const initEthers = async () => {
      if (window.ethereum) {
        const web3Provider = new BrowserProvider(window.ethereum);
        setProvider(web3Provider);
        try {
          const accounts = await web3Provider.listAccounts();
          if (accounts.length > 0) {
            setAccount(accounts[0].address);
            const signer = await web3Provider.getSigner();
            const sensorContract = new Contract(CONTRACT_ADDRESS, SensorDataRecorderAbi.abi, signer);
            setContract(sensorContract);
          }
        } catch (err) {
          console.error('Error checking existing connection:', err);
        }
      } else {
        setError('MetaMask is not installed. Please install MetaMask.');
      }
    };

    initEthers();

    if (window.ethereum) {
      window.ethereum.on('accountsChanged', (accounts: string[]) => {
        if (accounts.length === 0) {
          setAccount(null);
          setContract(null);
          setSensorReadings([]);
        } else {
          setAccount(accounts[0]);
          if (provider) {
            provider.getSigner().then(signer => {
              const sensorContract = new Contract(CONTRACT_ADDRESS, SensorDataRecorderAbi.abi, signer);
              setContract(sensorContract);
            }).catch(console.error);
          }
        }
      });
      window.ethereum.on('chainChanged', () => window.location.reload());
    }

    return () => {
      if (window.ethereum) {
        window.ethereum.removeListener('accountsChanged', () => {});
        window.ethereum.removeListener('chainChanged', () => {});
      }
    };
  }, [provider]);

  const fetchSensorData = useCallback(async () => {
    if (!contract) return;
    setLoading(true);
    setError(null);
    try {
      const rawData: any[] = await contract.getAllSensorData();
      const formattedData: SensorReading[] = rawData.map(item => ({
        ...item,
        temperatureCelsiusScaled: Number(item.temperatureCelsiusScaled) / 100,
        humidityPercentScaled: Number(item.humidityPercentScaled) / 100,
        timestampUnixNano: Number(item.timestampUnixNano)
      }));
      setSensorReadings([...formattedData].reverse());
    } catch (err: any) {
      console.error('Failed to fetch sensor data:', err);
      setError(`Failed to fetch data: ${err.message || err.toString()}`);
    } finally {
      setLoading(false);
    }
  }, [contract]);

  useEffect(() => {
    if (contract && account) fetchSensorData();
  }, [contract, account, fetchSensorData]);

  const handleConnectWallet = async () => {
    if (!provider || isConnecting) return;
    setIsConnecting(true);
    setError(null);
    try {
      const accounts = await provider.send('eth_requestAccounts', []);
      setAccount(accounts[0]);
      const signer = await provider.getSigner();
      const sensorContract = new Contract(CONTRACT_ADDRESS, SensorDataRecorderAbi.abi, signer);
      setContract(sensorContract);
    } catch (err: any) {
      console.error('Failed to connect to MetaMask:', err);
      if (err.code === 4001) {
        setError('Connection rejected by user.');
      } else {
        setError(`Failed to connect: ${err.message || err.toString()}`);
      }
    } finally {
      setIsConnecting(false);
    }
  };

  const exportToCSV = () => {
    const csv = Papa.unparse(sensorReadings.map(d => ({
      timestamp: new Date(d.timestampUnixNano / 1_000_000).toISOString(),
      sensorId: d.sensorId,
      temperature: d.temperatureCelsiusScaled,
      humidity: d.humidityPercentScaled
    })));
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
    saveAs(blob, 'sensor_data.csv');
  };

  return (
    <div style={{ padding: '40px', fontFamily: 'Segoe UI, sans-serif', backgroundColor: '#f4f6f8', minHeight: '100vh', color: '#333' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '30px', flexWrap: 'wrap' }}>
        <h1 style={{ fontWeight: '600', marginBottom: '10px' }}>Hydroponic Data Transaction</h1>
        {account ? (
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <span style={{ fontSize: '0.9rem', backgroundColor: '#ecf0f1', padding: '6px 10px', borderRadius: '6px', fontWeight: 500 }}>
              Connected: <strong>{account.slice(0, 6)}...{account.slice(-4)}</strong>
            </span>
            <button onClick={() => alert('Manually disconnect permissions from MetaMask.')} style={{ backgroundColor: '#e74c3c', color: 'white', border: 'none', borderRadius: '5px', padding: '8px 14px', cursor: 'pointer' }}>Disconnect</button>
          </div>
        ) : (
          <button onClick={handleConnectWallet} disabled={isConnecting} style={{ backgroundColor: '#3498db', color: 'white', border: 'none', borderRadius: '5px', padding: '10px 20px', cursor: 'pointer', fontWeight: 500 }}>{isConnecting ? 'Connecting...' : 'Connect Wallet'}</button>
        )}
      </div>

      <div style={{ marginBottom: '20px', display: 'flex', gap: '10px', flexWrap: 'wrap' }}>
        <button onClick={fetchSensorData} disabled={loading || !contract} style={{ backgroundColor: '#2ecc71', color: 'white', border: 'none', borderRadius: '5px', padding: '10px 20px', cursor: 'pointer' }}>{loading ? 'Loading...' : 'Refresh Data'}</button>
        <button onClick={exportToCSV} style={{ backgroundColor: '#f39c12', color: 'white', border: 'none', borderRadius: '5px', padding: '10px 20px', cursor: 'pointer' }}>Export CSV</button>
      </div>

      {sensorReadings.length > 0 && (
        <div style={{ marginBottom: '20px' }}>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={sensorReadings.slice().reverse()} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="timestampUnixNano" tickFormatter={(tick) => new Date(tick / 1_000_000).toLocaleTimeString()} />
              <YAxis yAxisId="left" domain={[0, 'auto']} label={{ value: '°C', angle: -90, position: 'insideLeft' }} />
              <YAxis yAxisId="right" orientation="right" domain={[0, 100]} label={{ value: '%', angle: 90, position: 'insideRight' }} />
              <Tooltip labelFormatter={(label) => new Date(label / 1_000_000).toLocaleString()} />
              <Legend />
              <Line yAxisId="left" type="monotone" dataKey="temperatureCelsiusScaled" stroke="#8e44ad" name="Temperature (°C)" dot={false} />
              <Line yAxisId="right" type="monotone" dataKey="humidityPercentScaled" stroke="#27ae60" name="Humidity (%)" dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      )}

      <div style={{ marginBottom: '30px' }}>
        <button onClick={() => setShowHistory(prev => !prev)} style={{ backgroundColor: '#34495e', width: '100%', color: 'white', border: 'none', borderRadius: '5px', padding: '12px 0', fontSize: '16px', cursor: 'pointer' }}>{showHistory ? 'Hide History' : 'View History'}</button>
      </div>

      {showHistory && (
        <div style={{ transition: 'all 0.3s ease-in-out' }}>
          {sensorReadings.length > 0 ? (
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '20px' }}>
              {sensorReadings.map((data, idx) => (
                <div key={idx} style={{ backgroundColor: '#fff', padding: '20px', borderRadius: '10px', boxShadow: '0 4px 8px rgba(0, 0, 0, 0.05)' }}>
                  <p><strong>Sensor ID:</strong> {data.sensorId}SHT20</p>
                  <p><strong>Timestamp:</strong> {new Date(data.timestampUnixNano / 1_000_000).toLocaleString('id-ID')}</p>
                  <p><strong>Temperature:</strong> {data.temperatureCelsiusScaled.toFixed(1)} °C</p>
                  <p><strong>Humidity:</strong> {data.humidityPercentScaled.toFixed(1)} %</p>
                </div>
              ))}
            </div>
          ) : (
            account && !loading && <p>📭 No sensor data stored on the blockchain yet.</p>
          )}
        </div>
      )}
    </div>
  );
};

export default App;
