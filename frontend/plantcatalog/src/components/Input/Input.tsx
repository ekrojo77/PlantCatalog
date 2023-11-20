import React from 'react';

interface InputProps {
  type: string;
  placeholder: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  value: string;
  id: string;
}

const Input: React.FC<InputProps> = ({ type, placeholder, onChange }) => {
  return (
    <input type={type} placeholder={placeholder} onChange={onChange} />
  );
};

export default Input;
